use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    GameStart,
    PlayerTurn,
    Identify,
    Roll,
    TimeToPlay,
    Move,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub action_type: Action,
    pub data: Option<String>,
}