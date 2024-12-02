use crate::board::Tile;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAP_GO: Vec<Tile> = vec![
        Tile::Go { amount: 200 },
        Tile::Go { amount: 100 },
        Tile::Go { amount: 300 }
    ];
}
