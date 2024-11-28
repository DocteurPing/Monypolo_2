use crate::game_state::{Game, WaitingRoom};
use std::collections::HashMap;
use tokio::sync::Mutex;
use uuid::Uuid;

pub(crate) struct ServerState {
    pub(crate) waiting_room: Mutex<WaitingRoom>,
    pub(crate) active_games: Mutex<HashMap<Uuid, Game>>,
}

impl ServerState {
    pub(crate) fn new() -> Self {
        ServerState {
            waiting_room: Mutex::new(WaitingRoom { players: vec![] }),
            active_games: Mutex::new(HashMap::new()),
        }
    }
}
