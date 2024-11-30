use crate::communication::send_to_all_players;
use crate::game_state::{Game, Player};
use shared::action::Action::PayRent;
use shared::action::{Action, BuyPropertyData, PayRentData};
use shared::board::Tile::Property;
use uuid::Uuid;

pub(crate) async fn roll_dice(game: &mut Game, uuid: uuid::Uuid) {
    println!("Player {} rolled the dice", uuid);
    // Generate random number between 2 and 12
    let roll = rand::random::<u8>() % 6 + 1 + rand::random::<u8>() % 6 + 1;
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
        Action::Move,
        Some(game.players[game.player_turn].position.to_string()),
    )
    .await;
    match &game.board[game.players[game.player_turn].position] {
        shared::board::Tile::Property {
            name,
            cost,
            rent,
            level,
            owner,
        } => {
            if owner.is_some() && owner.unwrap() != uuid {
                let rent_price = rent[level.clone() as usize];
                game.players[game.player_turn].money -= rent_price;
                let mut owner_player: &mut Player = game
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
                    player: uuid,
                };
                send_to_all_players(
                    &game.players,
                    PayRent,
                    Some(serde_json::to_string(&pay_rent_data).unwrap()),
                )
                .await;
            } else {
                send_to_all_players(
                    &game.players,
                    Action::AskBuyProperty,
                    Some(
                        serde_json::to_string(&BuyPropertyData {
                            position: game.players[game.player_turn].position as u32,
                            player: uuid,
                        })
                        .unwrap(),
                    ),
                )
                .await;
                return;
            }
        }
        shared::board::Tile::Chance { .. } => {}
        shared::board::Tile::Go => {}
        shared::board::Tile::Jail => {}
        shared::board::Tile::FreeParking => {}
        _ => {}
    }
    game.advance_turn().await;
}

pub(crate) async fn buy_property(uuid: Uuid, game: &mut Game) {
    // Buy property
    let tile = &mut game.board[game.players[game.player_turn].position];
    if let Property {
        ref mut owner,
        cost,
        ref mut level,
        ..
    } = tile
    {
        if owner.is_none() {
            let player = game.players.iter_mut().find(|p| p.id == uuid).unwrap();
            if player.money < cost[0] {
                println!("Player {} does not have enough money to buy property", uuid);
                game.advance_turn().await;
                return;
            }
            println!("Player {} bought property", uuid);
            player.money -= cost[0];
            *owner = Some(uuid);
            send_to_all_players(
                &game.players,
                Action::BuyProperty,
                Some(
                    serde_json::to_string(&BuyPropertyData {
                        player: uuid,
                        position: game.players[game.player_turn].position as u32,
                    })
                    .unwrap(),
                ),
            )
            .await;
            game.advance_turn().await;
        }
    }
}
