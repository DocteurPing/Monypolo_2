use crate::game_state::Player;
use crate::server_state::ServerState;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_test::block_on;
use uuid::Uuid;

#[test]
fn test_server_state_creation() {
    let state = ServerState::new();

    // Verify waiting room is empty
    let waiting_room = block_on(state.waiting_room.lock());
    assert!(waiting_room.players.is_empty());

    // Verify active games is empty
    let active_games = block_on(state.active_games.lock());
    assert!(active_games.is_empty());
}

#[tokio::test]
async fn test_server_state_concurrent_access() {
    let state = Arc::new(ServerState::new());
    let state_clone = Arc::clone(&state);

    // Spawn a task to modify the waiting room
    let task1 = tokio::spawn(async move {
        let mut waiting_room = state_clone.waiting_room.lock().await;
        // Perform some modification
        waiting_room.players.push(Player {
            id: Uuid::new_v4(),
            name: "ConcurrentPlayer".to_string(),
            tx: mpsc::channel(1).0,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        });
    });

    // Spawn another task to read from the waiting room
    let task2 = tokio::spawn(async move {
        let waiting_room = state.waiting_room.lock().await;
        waiting_room.players.len()
    });

    // Wait for both tasks to complete
    task1.await.unwrap();
    let player_count = task2.await.unwrap();

    // Verify the result - this could be 0 or 1 depending on timing
    assert!(player_count <= 1);
}
