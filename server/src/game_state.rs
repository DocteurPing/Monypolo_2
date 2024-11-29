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

impl GameState {
    pub(crate) fn advance_turn(&mut self) {
        self.current_turn += 1;
        self.player_turn = (self.player_turn + 1) % 4;
    }
}

pub(crate) async fn start_new_game(state: Arc<ServerState>) {
    let mut waiting_room = state.waiting_room.lock().await;
    let mut active_games = state.active_games.lock().await;

    if waiting_room.players.len() < 4 {
        return;
    }

    let mut players = waiting_room.players.drain(0..4).collect::<Vec<_>>();
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
        let message = format!("Game started! Your game ID is {}\n", game_id);
        let _ = player.tx.send(message).await;
    }
}



