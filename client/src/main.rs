mod tools;

use shared::action::{Action, PlayerAction};
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;
use tools::ToAction;

#[tokio::main]
async fn main() {
    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8080")
        .await
        .expect("Failed to connect to server");

    println!("Connected to server!");

    // Split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut buf = String::new();

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

async fn send_name(writer: &mut OwnedWriteHalf) {
    // Get the player's name
    println!("Enter your name:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let name = buf.trim();

    send_action(Action::Identify, Some(name.to_string()), writer).await;
}

pub async fn send_action(action: Action, data: Option<String>, writer: &mut OwnedWriteHalf) {
    // Send the player's action to the server
    // Create an action to send to the server
    println!("Sending action: {:?}", action);
    let action = PlayerAction {
        action_type: action,
        data, // Add specific data if required
    };

    let serialized_action = serde_json::to_string(&action).unwrap();
    if let Err(e) = writer
        .write_all((serialized_action + "\n").as_bytes())
        .await
    {
        eprintln!("Error sending action: {:?}", e);
    }
}
