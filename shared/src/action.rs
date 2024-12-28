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
    GoToJail,
    PlayerGoTile,
    FreeFromJail,
    PayTax
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerGoTileData {
    pub player: Uuid,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiceRollData {
    pub dice1: u8,
    pub dice2: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerIdentifyData {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerPayTaxData {
    pub player: Uuid,
    pub amount: u32,
}
