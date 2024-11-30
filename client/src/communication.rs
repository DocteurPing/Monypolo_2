use shared::action::{Action, PlayerAction};
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::OwnedWriteHalf;

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
