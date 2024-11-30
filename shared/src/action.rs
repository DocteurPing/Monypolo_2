use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Action {
    GameStart,
    PlayerTurn,
    Identify,
    Roll,
    TimeToPlay,
    PayRent,
    AskBuyProperty,
    BuyProperty,
    SkipBuyProperty,
    Move,
    Invalid,
    BuyAll,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAction {
    pub action_type: Action,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayRentData {
    pub rent: u32,
    pub owner: Uuid,
    pub player: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuyPropertyData {
    pub position: u32,
    pub player: Uuid,
}
