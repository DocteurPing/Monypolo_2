mod action;
mod communication;
mod game_state;
mod server_state;

use crate::communication::handle_connection;
use crate::server_state::ServerState;
use std::sync::Arc;
use tokio::net::TcpListener;

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
