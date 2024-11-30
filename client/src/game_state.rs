use shared::action::{Action, PlayerAction};
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) struct Player {
    pub(crate) money: u32,
    pub(crate) position: usize,
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
                    },
                );
            }
        }
        Action::PlayerTurn => {
            state.player_turn = action.data.unwrap().parse::<Uuid>().unwrap();
            println!("Player {} turn", state.player_turn);
        }
        Action::Identify => {
            let player_id = action.data.unwrap();
            state.id = player_id.parse::<Uuid>().unwrap();
            println!("Player identified with ID: {}", player_id);
        }
        Action::Move => {
            let roll = action.data.unwrap().parse::<u8>().unwrap();
            println!("uuid: {:?}", state.player_turn);
            state.players.get_mut(&state.player_turn).unwrap().position += roll as usize;
            println!(
                "Player moved to position {} tile {:?}",
                state.players.get(&state.player_turn).unwrap().position,
                state.board[state.players.get(&state.player_turn).unwrap().position]
            );
        }
        _ => {}
    }
}
