mod action;
mod communication;
mod game_state;
mod server_state;
mod test;

use crate::communication::handle_connection;
use crate::server_state::ServerState;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    env_logger::init();
    let state = Arc::new(ServerState::new());
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("Server running on 127.0.0.1:8080");
    log::debug!("Server started!");

    while let Ok((socket, _)) = listener.accept().await {
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            handle_connection(socket, state).await;
        });
    }
}
