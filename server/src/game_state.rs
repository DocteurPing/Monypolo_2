use crate::communication::{send_message, send_to_all_players};
use crate::server_state::ServerState;
use shared::maps::map1::MAP1;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) tx: mpsc::Sender<String>, // Channel to communicate with the player
    pub(crate) money: u32,
    pub(crate) position: usize,
}

#[derive(Debug)]
pub(crate) struct WaitingRoom {
    pub(crate) players: Vec<Player>,
}

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) id: Uuid,
    pub(crate) players: Vec<Player>,
    pub(crate) state: GameState,
}

#[derive(Debug)]
pub(crate) struct GameState {
    pub(crate) board: Vec<shared::board::Tile>,
    pub(crate) current_turn: usize,
    pub(crate) player_turn: usize,
}

impl Game {
    pub(crate) async fn advance_turn(&mut self) {
        self.state.current_turn += 1;
        self.state.player_turn = (self.state.player_turn + 1) % self.players.len();
        send_to_all_players(
            &self.players,
            shared::action::Action::PlayerTurn,
            Some(self.players[self.state.player_turn].id.to_string()),
        )
        .await;
    }
}

pub(crate) async fn start_new_game(state: Arc<ServerState>) {
    let mut waiting_room = state.waiting_room.lock().await;
    let mut active_games = state.active_games.lock().await;

    if waiting_room.players.len() < 1 {
        return;
    }

    let players = waiting_room.players.drain(0..2).collect::<Vec<_>>();
    let game_id = Uuid::new_v4();

    let game = Game {
        id: game_id,
        players: players.clone(),
        state: GameState {
            board: MAP1.to_vec(),
            current_turn: 0,
            player_turn: 0,
        },
    };

    active_games.insert(game_id, game);
    println!("Started a new game with ID: {}", game_id);

    for player in &players {
        // get all the ids of the players in the game in a string
        let player_ids = players
            .iter()
            .map(|p| p.id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        send_message(player, shared::action::Action::GameStart, Some(player_ids)).await;
    }
}
