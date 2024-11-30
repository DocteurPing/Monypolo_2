use crate::game_state::Player;
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn send_message(player: &Player, action: Action, data: Option<String>) {
    let action = PlayerAction {
        action_type: action,
        data, // Add specific data if required
    };

    let serialized_action = serde_json::to_string(&action).unwrap() + "\n";
    let _ = player.tx.send(serialized_action).await;
}

pub(crate) async fn handle_message_in_game(message: &String, state: &Arc<ServerState>, uuid: Uuid) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    let mut active_games = state.active_games.lock().await;
    for (_, game) in active_games.iter_mut() {
        if game.players[game.state.player_turn].id == uuid {
            match action.action_type {
                Action::Roll => {
                    println!("Player {} rolled the dice", uuid);
                    // Generate random number between 2 and 12
                    let roll = rand::random::<u8>() % 6 + 1 + rand::random::<u8>() % 6 + 1;
                    game.players[game.state.player_turn].position =
                        game.players[game.state.player_turn].position + roll as usize;
                    println!(
                        "Player {} moved to position {}",
                        uuid, game.players[game.state.player_turn].position
                    );
                    send_message(
                        &game.players[game.state.player_turn],
                        Action::Move,
                        Some(game.players[game.state.player_turn].position.to_string()),
                    )
                        .await;
                    game.state.advance_turn();
                }
                _ => {}
            }
        }
    }
}

pub(crate) async fn handle_message(message: &String, state: &Arc<ServerState>, uuid: Uuid) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        _ => {}
    }
}
