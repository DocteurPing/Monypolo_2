mod communication;
mod game_state;
mod tools;

use crate::communication::{send_action, send_name};
use crate::game_state::{handle_message_in_game, GamesState};
use shared::maps::map1::MAP1;
use std::collections::HashMap;
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tools::ToAction;

#[tokio::main]
async fn main() {
    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("Failed to connect to server");

    #[cfg(debug_assertions)]
    println!("Connected to server!");

    // Split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();
    let mut state = GamesState {
        id: Default::default(),
        players: HashMap::new(),
        current_turn: 0,
        player_turn: Default::default(),
        board: MAP1.clone(),
    };

    // Spawn a task to handle server messages
    tokio::spawn(async move {
        let mut buf = String::new();
        loop {
            buf.clear();
            match reader.read_line(&mut buf).await {
                Ok(0) => break, // Server closed connection
                Ok(_) => {
                    #[cfg(debug_assertions)]
                    println!("Server: {}", buf.trim());
                    handle_message_in_game(&buf, &mut state).await;
                    if buf.trim() == "Goodbye!" {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from server: {:?}", e);
                    break;
                }
            }
        }
    });

    send_name(&mut writer).await;

    // Main loop for user input
    loop {
        println!("Enter an action (e.g., roll, buy, quit):");
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();
        send_action(input.to_action(), None, &mut writer).await;

        if input == "quit" {
            writer.shutdown().await.unwrap();
            println!("Goodbye!");
            break;
        }
    }
}
