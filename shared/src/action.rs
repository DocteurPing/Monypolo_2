use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Identify,
    Roll,
    TimeToPlay,
    Invalid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub action_type: Action,
    pub data: Option<String>,
}