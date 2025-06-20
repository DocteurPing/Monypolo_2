use crate::game_state::{start_new_game, Game, Player, WaitingRoom};
use crate::server_state::ServerState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

#[tokio::test]
async fn test_advance_turn() {
    // Create a game with multiple players
    let mut game = Game::default();
    let (tx1, _) = mpsc::channel(32);
    let (tx2, _) = mpsc::channel(32);

    game.players = vec![
        Player {
            id: Uuid::new_v4(),
            name: "Player1".to_string(),
            tx: tx1,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
        Player {
            id: Uuid::new_v4(),
            name: "Player2".to_string(),
            tx: tx2,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
    ];

    // Initial player turn
    game.player_turn = 0;

    // Advance turn
    game.advance_turn().await;

    // Next player should be active
    assert_eq!(game.player_turn, 1);

    // Advance turn again, should wrap around
    game.advance_turn().await;
    assert_eq!(game.player_turn, 0);
}

#[tokio::test]
async fn test_advance_turn_with_bankrupt_player() {
    // Create a game with players including a bankrupt one
    let mut game = Game::default();
    let (tx1, _) = mpsc::channel(32);
    let (tx2, _) = mpsc::channel(32);
    let (tx3, _) = mpsc::channel(32);

    game.players = vec![
        Player {
            id: Uuid::new_v4(),
            name: "Player1".to_string(),
            tx: tx1,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
        Player {
            id: Uuid::new_v4(),
            name: "BankruptPlayer".to_string(),
            tx: tx2,
            money: 0,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: true, // This player is bankrupt
        },
        Player {
            id: Uuid::new_v4(),
            name: "Player3".to_string(),
            tx: tx3,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
    ];

    // Current player turn
    game.player_turn = 0;

    // Advance turn, should skip bankrupt player
    game.advance_turn().await;

    // Should skip player 1 (bankrupt) and go to player 2
    assert_eq!(game.player_turn, 2);
}

#[tokio::test]
async fn test_start_new_game() {
    // Create server state with enough players in waiting room
    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    let waiting_room = WaitingRoom {
        players: vec![
            Player {
                id: Uuid::new_v4(),
                name: "WaitingPlayer1".to_string(),
                tx: tx1,
                money: 1500,
                position: 0,
                is_in_jail: false,
                jail_turns: 0,
                is_bankrupt: false,
            },
            Player {
                id: Uuid::new_v4(),
                name: "WaitingPlayer2".to_string(),
                tx: tx2,
                money: 1500,
                position: 0,
                is_in_jail: false,
                jail_turns: 0,
                is_bankrupt: false,
            },
        ],
    };

    let state = Arc::new(ServerState {
        waiting_room: Mutex::new(waiting_room),
        active_games: Mutex::new(HashMap::new()),
    });

    // Start a new game
    start_new_game(Arc::clone(&state)).await;

    // Check that waiting room is empty
    let waiting_room = state.waiting_room.lock().await;
    assert!(waiting_room.players.is_empty());

    // Check that active games has one game
    let active_games = state.active_games.lock().await;
    assert_eq!(active_games.len(), 1);

    // Check that both players received game start message
    let msg1 = rx1.recv().await.unwrap();
    let msg2 = rx2.recv().await.unwrap();

    assert!(msg1.contains("GameStart"));
    assert!(msg2.contains("GameStart"));
}
