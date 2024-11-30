use crate::board::Tile;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAP_JAIL: Vec<Tile> = vec![Tile::Jail, Tile::GoToJail,];
}
