use crate::action::{buy_property, roll_dice};
use crate::game_state::{Game, Player};
use tokio::sync::mpsc;
use uuid::Uuid;

#[tokio::test]
async fn test_roll_dice() {
    // Create a test game
    let mut game = Game::default();
    let player_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::channel(32);

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

    // Initial position
    let initial_position = game.players[0].position;

    // Execute roll dice
    roll_dice(&mut game, &player_id).await;

    // Player should have moved (position changed)
    assert_ne!(game.players[0].position, initial_position);

    // Verify a message was sent to the player with roll data
    let message = rx.recv().await.unwrap();
    assert!(message.contains("Roll"));
    assert!(message.contains("dice1"));
    assert!(message.contains("dice2"));
}

#[tokio::test]
async fn test_roll_dice_in_jail() {
    // Create a test game with player in jail
    let mut game = Game::default();
    let player_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::channel(32);

    game.players = vec![Player {
        id: player_id,
        name: "JailedPlayer".to_owned(),
        tx,
        money: 1500,
        position: 10, // Jail position
        is_in_jail: true,
        jail_turns: 3,
        is_bankrupt: false,
    }];

    // Execute roll dice
    let (roll1, roll2) = roll_dice(&mut game, &player_id).await;

    if roll1 == roll2 {
        assert!(!game.players[0].is_in_jail);
    } else {
        // Player should still be in jail
        assert!(game.players[0].is_in_jail);

        // Jail turns should be decremented
        assert_eq!(game.players[0].jail_turns, 2);
    }

    // Messages should be received
    let _msg = rx.recv().await.unwrap();
}

#[tokio::test]
async fn test_buy_property() {
    // Create a test game with a player in position to buy property
    let mut game = Game::default();
    let player_id = Uuid::new_v4();
    let (tx, mut rx) = mpsc::channel(32);

    game.players = vec![Player {
        id: player_id,
        name: "Buyer".to_owned(),
        tx,
        money: 1500,
        position: 1, // Position with property
        is_in_jail: false,
        jail_turns: 0,
        is_bankrupt: false,
    }];

    // Get initial money
    let initial_money = game.players[0].money;

    // Execute buy property
    buy_property(player_id, &mut game).await;

    // Player should have less money now
    assert!(game.players[0].money < initial_money);

    // Property should be owned by player
    if let shared::board::Tile::Property { owner, .. } = game.board[1] {
        assert_eq!(owner, Some(player_id));
    } else {
        panic!("Expected property at position 1");
    }

    // Player should receive notification
    let msg = rx.recv().await.unwrap();
    assert!(msg.contains("BuyProperty"));
}
