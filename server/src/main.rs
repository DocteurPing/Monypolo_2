mod communication;
mod game_state;
mod server_state;

use crate::communication::{handle_message, handle_message_in_game, send_message};
use crate::game_state::{start_new_game, Player};
use crate::server_state::ServerState;
use shared::action::{Action, PlayerAction};
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
    send_message(&player, Action::Identify, Some(player_id.to_string())).await;

    // Add player to the waiting room
    {
        let mut waiting_room = state.waiting_room.lock().await;
        waiting_room.players.push(player);
        println!(
            "Player {} joined waiting room. Total players: {}",
            waiting_room.players.last().unwrap().name,
            waiting_room.players.len()
        );

        if waiting_room.players.len() == 2 {
            // Start a new game when there are 4 players
            tokio::spawn(start_new_game(Arc::clone(&state)));
        }
    }

    // Handle client messages
    loop {
        tokio::select! {
            Ok(len) = reader.read_line(&mut buf) => {
                let mut waiting_room = state.waiting_room.lock().await;
                if len == 0 {
                    println!("Player {} disconnected", player_id);
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
