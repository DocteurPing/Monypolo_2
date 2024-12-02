use shared::action::{Action, PlayerAction};
use shared::board::Tile::Property;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) money: u32,
    pub(crate) position: usize,
    pub(crate) is_in_jail: bool,
}

pub(crate) struct GamesState {
    pub(crate) id: Uuid,
    pub(crate) players: HashMap<Uuid, Player>,
    pub(crate) current_turn: usize,
    pub(crate) player_turn: Uuid,
    pub(crate) board: Vec<shared::board::Tile>,
}

pub(crate) async fn handle_message_in_game(message: &str, state: &mut GamesState) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        Action::GameStart => {
            start_game(state, action);
        }
        Action::PlayerTurn => {
            state.player_turn = action.data.unwrap().parse::<Uuid>().unwrap();
            println!("Player {} turn", state.player_turn);
        }
        Action::Identify => {
            state.id = action.data.unwrap().parse::<Uuid>().unwrap();
            println!("Player identified with ID: {}", state.id);
        }
        Action::Move => {
            let roll = action.data.unwrap().parse::<u8>().unwrap();
            println!("uuid: {:?}", state.player_turn);
            state.players.get_mut(&state.player_turn).unwrap().position = roll as usize;
            println!(
                "Player moved to position {} tile {:?}",
                state.players.get(&state.player_turn).unwrap().position,
                state.board[state.players.get(&state.player_turn).unwrap().position]
            );
        }
        Action::PayRent => {
            let data: shared::action::PayRentData =
                serde_json::from_str(&action.data.unwrap()).unwrap();
            let rent_price = data.rent;
            state.players.get_mut(&data.player).unwrap().money -= rent_price;
            state.players.get_mut(&data.owner).unwrap().money += rent_price;
            println!(
                "Player {} paid rent of {} to Player {}",
                data.player, rent_price, data.owner
            );
        }
        Action::AskBuyProperty => {
            let buy_property_data: shared::action::BuyPropertyData =
                serde_json::from_str(&action.data.unwrap()).unwrap();
            println!(
                "Player {} asked to buy property {}",
                buy_property_data.player, buy_property_data.position
            );
        }
        Action::BuyProperty => {
            let buy_property_data: shared::action::BuyPropertyData =
                serde_json::from_str(&action.data.unwrap()).unwrap();
            let player = state.players.get_mut(&buy_property_data.player).unwrap();
            // Get the property tile
            if let Property {
                ref mut owner,
                cost,
                ..
            } = &mut state.board[player.position]
            {
                player.money -= cost[0];
                *owner = Some(buy_property_data.player);
                println!(
                    "Player {} bought property {}",
                    buy_property_data.player, buy_property_data.position
                );
                println!("Property is now {:?}", state.board[player.position]);
            }
        }
        Action::GoToJail => {
            let player = state.players.get_mut(&state.player_turn).unwrap();
            player.position = state
                .board
                .iter()
                .position(|tile| matches!(tile, shared::board::Tile::Jail))
                .unwrap();
            player.is_in_jail = true;
            println!("Player {} is in jail", state.player_turn);
        }
        Action::PlayerGoTile => {
            let data =
                serde_json::from_str::<shared::action::PlayerGoTileData>(&action.data.unwrap())
                    .unwrap();
            let player = state.players.get_mut(&data.player).unwrap();
            player.money += data.amount;
            println!("Player {} got {} money", data.player, data.amount);
            println!(
                "Player {} money: {}",
                data.player,
                state.players.get_mut(&data.player).unwrap().money
            );
        }
        Action::Roll => {
            let data = serde_json::from_str::<shared::action::DiceRollData>(&action.data.unwrap())
                .unwrap();
            println!("Player rolled {} and {}", data.dice1, data.dice2);
        }
        _ => {}
    }
}

fn start_game(state: &mut GamesState, action: PlayerAction) {
    let data = action.data.unwrap();
    let players_id: Vec<&str> = data.split(',').collect();
    println!("Game started with {} players", players_id.len());
    println!("Players ID: {:?}", players_id);
    // Add a player for number of player stored in data
    for id in players_id {
        println!("Player {} joined the game", id.parse::<Uuid>().unwrap());
        state.players.insert(
            id.parse::<Uuid>().unwrap(),
            Player {
                money: 1500,
                position: 0,
                is_in_jail: false,
            },
        );
    }
}
