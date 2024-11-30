use crate::action::roll_dice;
use crate::game_state::Player;
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
use shared::board::Tile::Property;
use std::sync::Arc;
use uuid::Uuid;

pub(crate) async fn send_to_all_players(
    players: &Vec<Player>,
    action: Action,
    data: Option<String>,
) {
    for player in players {
        send_message(player, action.clone(), data.clone()).await;
    }
}
pub(crate) async fn send_message(player: &Player, action: Action, data: Option<String>) {
    let action = PlayerAction {
        action_type: action,
        data, // Add specific data if required
    };

    let serialized_action = serde_json::to_string(&action).unwrap() + "\n";
    let _ = player.tx.send(serialized_action).await;
}

pub(crate) async fn handle_message_in_game(message: &str, state: &Arc<ServerState>, uuid: Uuid) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    let mut active_games = state.active_games.lock().await;
    for (_, game) in active_games.iter_mut() {
        if game.players[game.player_turn].id == uuid {
            match action.action_type {
                Action::Roll => {
                    roll_dice(game, uuid).await;
                }
                Action::BuyAll => {
                    // Buy all properties
                    println!("Player {} bought all properties", uuid);
                    for tile in game.board.iter_mut() {
                        if let Property { ref mut owner, .. } = tile {
                            *owner = Some(uuid);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub(crate) async fn handle_message(message: &str, _state: &Arc<ServerState>, _uuid: Uuid) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        _ => {}
    }
}
