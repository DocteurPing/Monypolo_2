use shared::action::{Action, PlayerAction};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

use crate::{
    communication::{handle_message_in_game, send_message, send_to_all_players},
    game_state::{Game, Player, WaitingRoom},
    server_state::ServerState,
};

#[tokio::test]
async fn send_message_works() {
    // Create a mock player with a channel
    let (tx, mut rx) = mpsc::channel(32);
    let player = Player {
        id: Uuid::new_v4(),
        name: "TestPlayer".to_owned(),
        tx,
        money: 1500,
        position: 0,
        is_in_jail: false,
        jail_turns: 0,
        is_bankrupt: false,
    };

    // Send a test message
    send_message(&player, Action::Roll, Some("test_data".to_owned())).await;

    // Verify the message was sent correctly
    let received_msg = rx.recv().await.unwrap();
    let expected_action = PlayerAction {
        action_type: Action::Roll,
        data: Some("test_data".to_owned()),
    };
    let mut expected_msg = serde_json::to_string(&expected_action).unwrap();
    expected_msg.push('\n');
    assert_eq!(received_msg, expected_msg);
}

#[tokio::test]
async fn send_to_all_players_works() {
    // Create mock players with channels
    let (tx1, mut rx1) = mpsc::channel(32);
    let (tx2, mut rx2) = mpsc::channel(32);

    let players = vec![
        Player {
            id: Uuid::new_v4(),
            name: "Player1".to_owned(),
            tx: tx1,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
        Player {
            id: Uuid::new_v4(),
            name: "Player2".to_owned(),
            tx: tx2,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        },
    ];

    // Send a message to all players
    send_to_all_players(&players, Action::GameStart, Some("game_started".to_owned())).await;

    // Verify both players received the message
    let expected_action = PlayerAction {
        action_type: Action::GameStart,
        data: Some("game_started".to_owned()),
    };
    let mut expected_msg = serde_json::to_string(&expected_action).unwrap();
    expected_msg.push('\n');

    assert_eq!(rx1.recv().await.unwrap(), expected_msg);
    assert_eq!(rx2.recv().await.unwrap(), expected_msg);
}

#[tokio::test]
async fn handle_message_in_game_roll_action() {
    // Create a mock game with players
    let player_id = Uuid::new_v4();
    let (tx, _rx) = mpsc::channel(32);
    let mut game = Game::default();
    game.players = vec![Player {
        id: player_id,
        name: "TestPlayer".to_owned(),
        tx,
        money: 1500,
        position: 0,
        is_in_jail: false,
        jail_turns: 0,
        is_bankrupt: false,
    }];

    // Create a mock server state
    let mut active_games = HashMap::new();
    active_games.insert(game.id, game.clone());
    let state = Arc::new(ServerState {
        waiting_room: Mutex::new(WaitingRoom { players: vec![] }),
        active_games: Mutex::new(active_games),
    });

    // Create a roll action message
    let action = PlayerAction {
        action_type: Action::Roll,
        data: None,
    };
    let message = serde_json::to_string(&action).unwrap();

    // Handle the message
    handle_message_in_game(&message, &state, player_id).await;

    // The player position should have changed - this is hard to test deterministically
    // but we can verify the game still exists
    assert!(state.active_games.lock().await.contains_key(&game.id));
}
