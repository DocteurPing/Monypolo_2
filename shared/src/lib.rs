pub mod action;

use serde::{Deserialize, Serialize};
use crate::action::Action;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub action_type: Action,
    pub data: Option<String>,
}
