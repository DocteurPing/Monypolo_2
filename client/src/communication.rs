use crate::game_state::{handle_message_in_game, GamesState};
use async_channel::{unbounded, Receiver, Sender};
use bevy::prelude::{AssetServer, Commands, Deref, DerefMut, Res, ResMut, Resource};
use shared::action::{Action, PlayerAction};
use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpStream;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct MessageReceiver(pub Receiver<String>);

#[derive(Resource, Clone)]
pub(crate) struct MessageSender(pub Sender<PlayerAction>);

pub(crate) async fn send_name(writer: &mut OwnedWriteHalf) {
    // Get the player's name
    println!("Enter your name:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let name = buf.trim();

    send_action(Action::Identify, Some(name.to_string()), writer).await;
}

pub(crate) async fn send_action(action: Action, data: Option<String>, writer: &mut OwnedWriteHalf) {
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

pub(crate) async fn connect_to_server(
    tx_server: Sender<String>,
    rx_client: Receiver<PlayerAction>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    #[cfg(debug_assertions)]
    println!("Connected to server!");

    // Split the stream into reader and writer
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    send_name(&mut writer).await;

    // Spawn a task to handle server messages
    let mut buf = String::new();
    loop {
        buf.clear();
        match reader.read_line(&mut buf).await {
            Ok(0) => {
                eprintln!("Server closed connection");
                return Err("Server closed connection".into());
            } // Server closed connection
            Ok(_) => {
                #[cfg(debug_assertions)]
                println!("Server: {}", buf.trim());
                tx_server.send(buf.clone()).await.unwrap();
                //handle_message_in_game(&buf, &mut state).await;
                if buf.trim() == "Goodbye!" {
                    return Ok(());
                }
            }
            Err(e) => {
                eprintln!("Error reading from server: {:?}", e);
                return Err(e.into());
            }
        }
        if let Ok(action) = rx_client.try_recv() {
            println!("Sending message: {:?}", action.action_type);
            send_action(action.action_type, action.data, &mut writer).await;
        }
    }
}

pub(crate) fn receive_message(
    receiver: ResMut<MessageReceiver>,
    sender: ResMut<MessageSender>,
    mut game_state: ResMut<GamesState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(message) = receiver.0.try_recv() {
        println!("Processing message: {}", message.trim());
        // Update game state based on the received message
        // Example: Parse and handle the message
        handle_message_in_game(
            &message,
            &mut game_state,
            sender.0.clone(),
            &mut commands,
            &asset_server,
        );
    }
}

pub(crate) async fn setup_network() -> (Receiver<String>, Sender<PlayerAction>) {
    let (tx_server, rx_server) = unbounded();
    let (tx_client, rx_client) = unbounded::<PlayerAction>();

    // Spawn the Tokio task for network communication
    tokio::spawn(async move {
        if let Err(e) = connect_to_server(tx_server, rx_client).await {
            eprintln!("Error in network communication: {e:?}");
        }
    });
    (rx_server, tx_client)
}
