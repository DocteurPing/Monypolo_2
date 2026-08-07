use crate::action::{buy_property, roll_dice};
use crate::game_state::{start_new_game, Player};
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
use shared::board::Tile::Property;
use shared::list_const::NUMBER_PLAYERS_PER_GAME;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
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
    let mut serialized_action = serde_json::to_string(&action).unwrap();
    serialized_action.push('\n');
    let _ = player.tx.send(serialized_action).await;
}

pub(crate) async fn handle_message_in_game(message: &str, state: &Arc<ServerState>, uuid: Uuid) {
    let action: PlayerAction = match serde_json::from_str(message) {
        Ok(action) => action,
        Err(e) => {
            log::warn!(
                "Player {uuid} sent an invalid message ({e}): {}",
                message.trim()
            );
            return;
        }
    };
    let mut active_games = state.active_games.lock().await;
    for game in active_games.values_mut() {
        if game.players[game.player_turn].id == uuid {
            match action.action_type {
                Action::Roll => {
                    roll_dice(game, &uuid).await;
                }
                Action::BuyProperty => {
                    buy_property(uuid, game).await;
                }
                Action::SkipBuyProperty => {
                    log::debug!("Player {uuid} skipped buying property");
                    send_to_all_players(
                        &game.players,
                        Action::SkipBuyProperty,
                        Some(game.players[game.player_turn].id.to_string()),
                    )
                    .await;
                    game.advance_turn().await;
                }
                Action::BuyAll => {
                    // Buy all properties for debug purpose only
                    log::debug!("Player {uuid} bought all properties");
                    for tile in &mut game.board {
                        if let Property { owner, .. } = tile {
                            *owner = Some(uuid);
                        }
                    }
                }
                _ => {}
            }
            break;
        }
    }
}

pub(crate) async fn handle_message(message: &str, _state: &Arc<ServerState>, uuid: Uuid) {
    if let Err(e) = serde_json::from_str::<PlayerAction>(message) {
        log::warn!(
            "Player {uuid} sent an invalid message ({e}): {}",
            message.trim()
        );
    }
}

pub(crate) async fn handle_connection(socket: TcpStream, state: Arc<ServerState>) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

    match reader.read_line(&mut buf).await {
        Ok(0) => {
            log::debug!("Player disconnected before identifying");
            return;
        }
        Ok(_) => {}
        Err(e) => {
            log::error!("Error reading identify message: {e}");
            return;
        }
    }
    let player_action: PlayerAction = match serde_json::from_str(&buf) {
        Ok(action) => action,
        Err(e) => {
            log::error!("Invalid identify message ({e}): {}", buf.trim());
            return;
        }
    };
    buf.clear();
    let Some(name) = player_action.data else {
        log::error!("Player did not provide a name in identify message");
        return;
    };

    let (tx, mut rx) = mpsc::channel(32); // Player's message channel
    let player = Player::default(tx, name);
    let player_id = player.id;
    send_message(&player, Action::Identify, Some(player_id.to_string())).await;

    add_to_waiting_room(&state, player).await;

    // Handle client messages
    loop {
        tokio::select! {
            result = reader.read_line(&mut buf) => {
                let len = match result {
                    Ok(len) => len,
                    Err(e) => {
                        log::error!("Error reading from player {player_id}: {e}");
                        break;
                    }
                };
                let mut waiting_room = state.waiting_room.lock().await;
                if len == 0 {
                    log::debug!("Player {player_id} disconnected");
                    if waiting_room.players.iter().any(|p| p.id == player_id) {
                        waiting_room.players.retain(|player| player.id != player_id);
                        log::debug!("Player {} left waiting room. Total players: {}",player_id, waiting_room.players.len());
                        log::debug!("Players {:?}", waiting_room.players);
                    } else {
                        let mut games = state.active_games.lock().await;
                        for game in games.values_mut() {
                            let is_player_turn = game.players[game.player_turn].id == player_id;
                            game.players.retain(|player| player.id != player_id);
                            log::debug!("Player {player_id} left the game. Total player in the game: {}", game.players.len());
                            if is_player_turn && !game.players.is_empty() {
                                game.advance_turn().await;
                            }
                        }
                        games.retain(|_, game| {
                            let retain_game = !game.players.is_empty() && game.is_active;
                            if !retain_game {
                                log::debug!("Game ended due to player leaving");
                            }
                            retain_game
                        });
                    }
                    break;
                }
                log::debug!("Received message: {}", buf.trim());
                if waiting_room.players.iter().any(|player| player.id == player_id) {
                    handle_message(&buf, &state, player_id).await;
                } else {
                    handle_message_in_game(&buf, &state, player_id).await;
                }
                buf.clear();
            }
            Some(msg) = rx.recv() => {
                if let Err(e) = writer.write_all(msg.as_bytes()).await {
                    log::error!("Error writing to player {player_id}: {e}");
                    break;
                }
            }
        }
    }
}

async fn add_to_waiting_room(state: &Arc<ServerState>, player: Player) {
    // Add player to the waiting room
    let mut waiting_room = state.waiting_room.lock().await;
    waiting_room.players.push(player);
    log::debug!(
        "Player {} joined waiting room id: {}. Total players: {}",
        waiting_room.players.last().unwrap().name,
        waiting_room.players.last().unwrap().id,
        waiting_room.players.len()
    );

    if waiting_room.players.len() == NUMBER_PLAYERS_PER_GAME {
        // Start a new game when there are enough players
        tokio::spawn(start_new_game(Arc::clone(state)));
    }
}
