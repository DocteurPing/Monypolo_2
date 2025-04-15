use crate::communication::send_to_all_players;
use crate::game_state::{Game, Player};
use serde_json::to_string;
use shared::action::Action::PayRent;
use shared::action::{
    Action, BuyPropertyData, DiceRollData, PayRentData, PlayerGoTileData, PlayerPayTaxData,
};
use shared::board::Tile::*;
use uuid::Uuid;

pub(crate) async fn roll_dice(game: &mut Game, uuid: Uuid) {
    println!("Player {} rolled the dice", uuid);
    // Generate random number between 2 and 12
    let roll1 = rand::random::<u8>() % 6 + 1;
    let roll2 = rand::random::<u8>() % 6 + 1;
    let roll = roll1 + roll2;
    if game.players[game.player_turn].is_in_jail {
        println!("Player {} is in jail", uuid);
        if roll1 == roll2 {
            game.players[game.player_turn].is_in_jail = false;
            game.players[game.player_turn].jail_turns = 0;
            println!("Player {} rolled doubles and is out of jail", uuid);
            send_to_all_players(
                &game.players,
                Action::FreeFromJail,
                Some(game.players[game.player_turn].id.to_string()),
            )
            .await;
        } else {
            game.players[game.player_turn].jail_turns -= 1;
            if game.players[game.player_turn].jail_turns == 0 {
                game.players[game.player_turn].is_in_jail = false;
                println!("Player {} is out of jail", uuid);
            } else {
                println!("Player {} is still in jail", uuid);
            }
            game.advance_turn().await;
            return;
        }
    }
    game.players[game.player_turn].position =
        (game.players[game.player_turn].position + roll as usize) % game.board.len();
    println!(
        "Player {} moved to position {}",
        uuid, game.players[game.player_turn].position
    );
    println!(
        "Tile: {:?}",
        game.board[game.players[game.player_turn].position]
    );
    send_to_all_players(
        &game.players,
        Action::Roll,
        Some(
            to_string(&DiceRollData {
                dice1: roll1,
                dice2: roll2,
            })
            .unwrap(),
        ),
    )
    .await;
    send_to_all_players(
        &game.players,
        Action::Move,
        Some(game.players[game.player_turn].position.to_string()),
    )
    .await;
    match game.board[game.players[game.player_turn].position].clone() {
        Property {
            rents,
            level,
            owner,
            costs,
            ..
        } => {
            pay_rent_or_buy(game, &uuid, rents[level.clone() as usize], &owner, costs[0]).await;
            return;
        }
        Railroad {
            owner, rents, cost, ..
        } => {
            let rent = get_rent_railroad(rents, &owner, game);
            pay_rent_or_buy(game, &uuid, rent, &owner, cost).await;
            return;
        }
        Chance { .. } => {}
        Go { amount } => {
            game.players[game.player_turn].money += amount;
            send_to_all_players(
                &game.players,
                Action::PlayerGoTile,
                Some(
                    to_string(&PlayerGoTileData {
                        player: game.players[game.player_turn].id,
                        amount,
                    })
                    .unwrap(),
                ),
            )
            .await;
        }
        Jail => {}
        GoToJail => {
            game.players[game.player_turn].position = game
                .board
                .iter()
                .position(|tile| matches!(tile, Jail))
                .unwrap();
            game.players[game.player_turn].is_in_jail = true;
            game.players[game.player_turn].jail_turns = 3;
            send_to_all_players(
                &game.players,
                Action::GoToJail,
                Some(game.players[game.player_turn].id.to_string()),
            )
            .await;
        }
        FreeParking => {}
        Utility { owner, cost, .. } => {
            let rent = calculate_utility_cost(roll, &owner, game);
            pay_rent_or_buy(game, &uuid, rent, &owner, cost).await;
            return;
        }
        Tax { price } | LuxuryTax { price } => {
            if game.players[game.player_turn].money < price {
                println!("Player {} does not have enough money to pay tax", uuid);
                game.players[game.player_turn].is_bankrupt = true;
                send_to_all_players(
                    &game.players,
                    Action::PlayerBankrupt,
                    Some(uuid.to_string()),
                )
                .await;
                game.advance_turn().await;
                return;
            }
            game.players[game.player_turn].money -= price;
            send_to_all_players(
                &game.players,
                Action::PayTax,
                Some(
                    to_string(&PlayerPayTaxData {
                        player: game.players[game.player_turn].id,
                        amount: price,
                    })
                    .unwrap(),
                ),
            )
            .await;
        }
    }
    game.advance_turn().await;
}

fn calculate_utility_cost(dice_roll: u8, owner: &Option<Uuid>, game: &mut Game) -> u32 {
    if let Some(owner) = owner {
        let number_utilities = game.board.iter().filter(|tile| matches!(tile, Utility { owner: Some(tile_owner), .. } if tile_owner == owner)).count() as u32;
        return match number_utilities {
            1 => 4 * dice_roll as u32,
            2 => 10 * dice_roll as u32,
            _ => 0,
        };
    }
    0
}

fn get_rent_railroad(rent: Vec<u32>, owner: &Option<Uuid>, game: &mut Game) -> u32 {
    if let Some(owner_id) = owner {
        return rent[game
            .board
            .iter()
            .filter(|tile| matches!(tile, Railroad { owner: Some(tile_owner), .. } if tile_owner == owner_id))
            .count()];
    }
    0
}

async fn pay_rent_or_buy(
    game: &mut Game,
    uuid: &Uuid,
    rent_price: u32,
    owner: &Option<Uuid>,
    cost: u32,
) {
    if owner.is_some() && owner.unwrap() != *uuid {
        if game.players[game.player_turn].money < rent_price {
            println!("Player {} does not have enough money to pay rent", uuid);
            game.players[game.player_turn].is_bankrupt = true;
            send_to_all_players(
                &game.players,
                Action::PlayerBankrupt,
                Some(uuid.to_string()),
            )
            .await;
            game.advance_turn().await;
            return;
        }
        game.players[game.player_turn].money -= rent_price;
        let owner_player: &mut Player = game
            .players
            .iter_mut()
            .find(|player| player.id == owner.unwrap())
            .unwrap();
        owner_player.money += rent_price;
        println!(
            "Player {} paid rent of {} to Player {}",
            uuid, rent_price, owner_player.id
        );
        let pay_rent_data = PayRentData {
            rent: rent_price,
            owner: owner_player.id,
            player: *uuid,
        };
        send_to_all_players(
            &game.players,
            PayRent,
            Some(to_string(&pay_rent_data).unwrap()),
        )
        .await;
    } else if game.players[game.player_turn].money >= cost {
        send_to_all_players(
            &game.players,
            Action::AskBuyProperty,
            Some(
                to_string(&BuyPropertyData {
                    position: game.players[game.player_turn].position as u32,
                    player: *uuid,
                })
                .unwrap(),
            ),
        )
        .await;
        return;
    }
    game.advance_turn().await;
}

pub(crate) async fn buy_property(uuid: Uuid, game: &mut Game) {
    let tile = &mut game.board[game.players[game.player_turn].position];
    let player = game.players.iter_mut().find(|p| p.id == uuid).unwrap();

    match tile {
        Property {
            ref mut owner,
            costs,
            ..
        } if owner.is_none() => {
            if player.money >= costs[0] {
                player.money -= costs[0];
                *owner = Some(uuid);
                println!(
                    "Player {} bought property money of the player {}",
                    uuid, player.money
                );
                send_to_all_players(
                    &game.players,
                    Action::BuyProperty,
                    Some(
                        to_string(&BuyPropertyData {
                            player: uuid,
                            position: game.players[game.player_turn].position as u32,
                        })
                        .unwrap(),
                    ),
                )
                .await;
            } else {
                println!("Player {} does not have enough money to buy property", uuid);
            }
        }
        Railroad {
            ref mut owner,
            cost,
            ..
        }
        | Utility {
            ref mut owner,
            cost,
        } if owner.is_none() => {
            if player.money >= *cost {
                player.money -= *cost;
                *owner = Some(uuid);
                println!(
                    "Player {} bought property money of the player {}",
                    uuid, player.money
                );
                send_to_all_players(
                    &game.players,
                    Action::BuyProperty,
                    Some(
                        to_string(&BuyPropertyData {
                            player: uuid,
                            position: game.players[game.player_turn].position as u32,
                        })
                        .unwrap(),
                    ),
                )
                .await;
            } else {
                println!("Player {} does not have enough money to buy property", uuid);
            }
        }
        _ => {}
    }
    game.advance_turn().await;
}
