use crate::game_state::{handle_message_in_game, GamesState};
use crate::ui::toast::ToastCount;
use async_channel::{unbounded, Receiver, Sender};
use bevy::prelude::{
    AssetServer, Commands, Deref, DerefMut, Query, Res, ResMut, Resource, Transform,
};
use shared::action::{Action, PlayerAction};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct MessageReceiver(pub Receiver<String>);

#[derive(Resource, Clone)]
pub(crate) struct MessageSender(pub Sender<PlayerAction>);

pub(crate) async fn send_action(action: Action, data: Option<String>, writer: &mut OwnedWriteHalf) {
    // Send the player's action to the server
    // Create an action to send to the server
    log::debug!("Sending action: {:?}", action);
    let action = PlayerAction {
        action_type: action,
        data, // Add specific data if required
    };

    let serialized_action = serde_json::to_string(&action).unwrap();
    if let Err(e) = writer
        .write_all((serialized_action + "\n").as_bytes())
        .await
    {
        log::error!("Error sending action: {:?}", e);
    }
}

pub(crate) async fn connect_to_server() -> (BufReader<OwnedReadHalf>, OwnedWriteHalf) {
    // Connect to the server
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    #[cfg(debug_assertions)]
    log::debug!("Connected to server!");

    // Split the stream into reader and writer
    let (reader, writer) = stream.into_split();
    let reader = BufReader::new(reader);

    (reader, writer)
}

async fn handle_server_communication(
    tx_server: Sender<String>,
    rx_client: Receiver<PlayerAction>,
    mut reader: BufReader<OwnedReadHalf>,
    mut writer: OwnedWriteHalf,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    loop {
        tokio::select! {
            // Read from the server
            result = reader.read_line(&mut buf) => {
                match result {
                    Ok(0) => {
                        log::error!("Server closed connection");
                        return Err("Server closed connection".into());
                    }
                    Ok(_) => {
                        #[cfg(debug_assertions)]
                        log::debug!("Server: {}", buf.trim());
                        tx_server.send(buf.clone()).await.unwrap();
                        if buf.trim() == "Goodbye!" {
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        log::error!("Error reading from server: {:?}", e);
                        return Err(e.into());
                    }
                }
                buf.clear();
            }
            // Write to the server
            action = rx_client.recv() => {
                match action {
                    Ok(action) => {
                        log::debug!("Sending message: {:?}", action.action_type);
                        send_action(action.action_type, action.data, &mut writer).await;
                    }
                    Err(e) => {
                        log::debug!("No message to send");
                        return Err(e.into());
                    }
                }
            }
        }
    }
}

pub(crate) fn receive_message(
    receiver: ResMut<MessageReceiver>,
    mut game_state: ResMut<GamesState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    transforms: Query<&mut Transform>,
    toast_count: ResMut<ToastCount>,
) {
    if let Ok(message) = receiver.0.try_recv() {
        log::debug!("Processing message: {}", message.trim());
        // Update game state based on the received message
        // Example: Parse and handle the message
        handle_message_in_game(
            &message,
            &mut game_state,
            &mut commands,
            &asset_server,
            transforms,
            toast_count,
        );
    }
}

pub(crate) async fn setup_network() -> (Receiver<String>, Sender<PlayerAction>) {
    let (tx_server, rx_server) = unbounded();
    let (tx_client, rx_client) = unbounded::<PlayerAction>();

    // Spawn the Tokio task for network communication
    tokio::spawn(async move {
        let (reader, writer) = connect_to_server().await;
        if let Err(e) = handle_server_communication(tx_server, rx_client, reader, writer).await {
            log::error!("Error in server communication: {:?}", e);
        }
    });
    (rx_server, tx_client)
}
