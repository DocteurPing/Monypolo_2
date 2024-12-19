use crate::action::{buy_property, roll_dice};
use crate::game_state::{start_new_game, Player};
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
use shared::board::Tile::Property;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
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
                Action::BuyProperty => {
                    buy_property(uuid, game).await;
                }
                Action::SkipBuyProperty => {
                    println!("Player {} skipped buying property", uuid);
                    game.advance_turn().await;
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

pub(crate) async fn handle_connection(socket: tokio::net::TcpStream, state: Arc<ServerState>) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

    reader.read_line(&mut buf).await.unwrap();
    let player_action: PlayerAction = serde_json::from_str(&buf).unwrap();
    buf.clear();
    let name = player_action.data.unwrap();

    let (tx, mut rx) = mpsc::channel(32); // Player's message channel
    let player = Player::default(tx, name);
    let player_id = player.id;
    send_message(&player, Action::Identify, Some(player_id.to_string())).await;

    add_to_waiting_room(&state, player).await;

    // Handle client messages
    loop {
        tokio::select! {
            Ok(len) = reader.read_line(&mut buf) => {
                let mut waiting_room = state.waiting_room.lock().await;
                if len == 0 {
                    println!("Player {} disconnected", player_id);
                    println!("Players {:?}", waiting_room.players);
                    waiting_room.players.retain(|p| p.id != player_id);
                    println!("Player {} left waiting room. Total players: {}",player_id,waiting_room.players.len());
                    break;
                }
                println!("Received message: {}", buf.trim());
                if waiting_room.players.iter().any(|player| player.id == player_id) {
                    handle_message(&buf, &state, player_id).await;
                } else {
                    handle_message_in_game(&buf, &state, player_id).await;
                }
                buf.clear();
            }
            Some(msg) = rx.recv() => {
                writer.write_all(msg.as_bytes()).await.unwrap();
            }
        }
    }
}

async fn add_to_waiting_room(state: &Arc<ServerState>, player: Player) {
    // Add player to the waiting room
    let mut waiting_room = state.waiting_room.lock().await;
    waiting_room.players.push(player);
    println!(
        "Player {} joined waiting room id: {}. Total players: {}",
        waiting_room.players.last().unwrap().name,
        waiting_room.players.last().unwrap().id,
        waiting_room.players.len()
    );

    if waiting_room.players.len() == 2 {
        // Start a new game when there are 4 players
        tokio::spawn(start_new_game(Arc::clone(state)));
    }
}
