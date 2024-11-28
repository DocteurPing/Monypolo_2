use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub action_type: String,
    pub data: Option<String>,
}
