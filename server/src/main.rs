mod game_state;
mod server_state;

use crate::game_state::{Game, GameState, Player};
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
use shared::maps::map1::MAP1;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let state = Arc::new(ServerState::new());
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Server running on 127.0.0.1:8080");

    while let Ok((socket, _)) = listener.accept().await {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            handle_connection(socket, state).await;
        });
    }
}

async fn handle_connection(socket: tokio::net::TcpStream, state: Arc<ServerState>) {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

    // Generate a unique ID for the player
    let player_id = Uuid::new_v4();

    reader.read_line(&mut buf).await.unwrap();
    let player_action: PlayerAction = serde_json::from_str(&buf).unwrap();
    buf.clear();
    let name = player_action.data.unwrap();

    let (tx, mut rx) = mpsc::channel(32); // Player's message channel
    let player = Player {
        id: player_id,
        name,
        tx,
        money: 1500,
        position: 0,
    };

    // Add player to the waiting room
    {
        let mut waiting_room = state.waiting_room.lock().await;
        waiting_room.players.push(player);
        println!(
            "Player {} joined waiting room. Total players: {}",
            waiting_room.players.last().unwrap().name,
            waiting_room.players.len()
        );

        if waiting_room.players.len() == 4 {
            // Start a new game when there are 4 players
            tokio::spawn(start_new_game(Arc::clone(&state)));
        }
    }

    // Handle client messages
    loop {
        tokio::select! {
            Ok(len) = reader.read_line(&mut buf) => {
                if len == 0 {
                    println!("Player {} disconnected", player_id);
                    break;
                }
                println!("Received message: {}", buf.trim());
                handle_message(&buf, &state, player_id).await;
                buf.clear();
            }
            Some(msg) = rx.recv() => {
                writer.write_all(msg.as_bytes()).await.unwrap();
            }
        }
    }
}

async fn handle_message(message: &String, state: &Arc<ServerState>, uuid: Uuid) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        Action::Quit => {
            let mut waiting_room = state.waiting_room.lock().await;
            waiting_room.players.retain(|player| player.id != uuid);
            println!(
                "Player {} left waiting room. Total players: {}",
                uuid,
                waiting_room.players.len()
            );
        }
        _ => {}
    }
}

async fn start_new_game(state: Arc<ServerState>) {
    let mut waiting_room = state.waiting_room.lock().await;
    let mut active_games = state.active_games.lock().await;

    if waiting_room.players.len() < 4 {
        return;
    }

    let players = waiting_room.players.drain(0..4).collect::<Vec<_>>();
    let game_id = Uuid::new_v4();

    let game = Game {
        id: game_id,
        players: players.clone(),
        state: GameState {
            board: MAP1.to_vec(),
            current_turn: 0,
        },
    };

    active_games.insert(game_id, game);
    println!("Started a new game with ID: {}", game_id);

    for player in players {
        let message = format!("Game started! Your game ID is {}\n", game_id);
        let _ = player.tx.send(message).await;
    }
}
