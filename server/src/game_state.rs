use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) tx: mpsc::Sender<String>, // Channel to communicate with the player
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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct GameState {
    pub(crate) board: Vec<Tile>,
    pub(crate) current_turn: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Tile {
    Property { name: String, cost: u32, rent: u32, owner: Option<Uuid> },
    Chance(String),
    Jail,
    Go,
    FreeParking,
}
