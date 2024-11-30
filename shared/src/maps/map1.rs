use crate::board::{PropertyLevel, Tile};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAP1: Vec<Tile> = vec![
        Tile::Go,
        Tile::Property {
            name: "Mediterranean Avenue".to_string(),
            cost: vec![60],
            rent: vec![2, 10, 30, 90, 160, 250],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Chance("Community Chest".to_string()),
        Tile::Property {
            name: "Baltic Avenue".to_string(),
            cost: vec![60],
            rent: vec![4, 20, 60, 180, 320, 450],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Tax,
        Tile::Railroad,
        Tile::Property {
            name: "Oriental Avenue".to_string(),
            cost: vec![100],
            rent: vec![6, 30, 90, 270, 400, 550],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Chance("Chance".to_string()),
        Tile::Property {
            name: "Vermont Avenue".to_string(),
            cost: vec![100],
            rent: vec![6, 30, 90, 270, 400, 550],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "Connecticut Avenue".to_string(),
            cost: vec![120],
            rent: vec![8, 40, 100, 300, 450, 600],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Jail,
        Tile::Property {
            name: "St. Charles Place".to_string(),
            cost: vec![140],
            rent: vec![10, 50, 150, 450, 625, 750],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Utility,
        Tile::Property {
            name: "States Avenue".to_string(),
            cost: vec![140],
            rent: vec![10, 50, 150, 450, 625, 750],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "Virginia Avenue".to_string(),
            cost: vec![160],
            rent: vec![12, 60, 180, 500, 700, 900],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Railroad,
        Tile::Property {
            name: "St. James Place".to_string(),
            cost: vec![180],
            rent: vec![14, 70, 200, 550, 750, 950],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Chance("Community Chest".to_string()),
        Tile::Property {
            name: "Tennessee Avenue".to_string(),
            cost: vec![180],
            rent: vec![14, 70, 200, 550, 750, 950],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "New York Avenue".to_string(),
            cost: vec![200],
            rent: vec![16, 80, 220, 600, 800, 1000],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::FreeParking,
        Tile::Property {
            name: "Kentucky Avenue".to_string(),
            cost: vec![220],
            rent: vec![18, 90, 250, 700, 875, 1050],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Chance("Community Chest".to_string()),
        Tile::Property {
            name: "Indiana Avenue".to_string(),
            cost: vec![220],
            rent: vec![18, 90, 250, 700, 875, 1050],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "Illinois Avenue".to_string(),
            cost: vec![240],
            rent: vec![20, 100, 300, 750, 925, 1100],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Railroad,
        Tile::Property {
            name: "Atlantic Avenue".to_string(),
            cost: vec![260],
            rent: vec![22, 110, 330, 800, 975, 1150],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "Ventnor Avenue".to_string(),
            cost: vec![260],
            rent: vec![22, 110, 330, 800, 975, 1150],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Utility,
        Tile::Property {
            name: "Marvin Gardens".to_string(),
            cost: vec![280],
            rent: vec![24, 120, 360, 850, 1025, 1200],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Jail,
        Tile::Property {
            name: "Pacific Avenue".to_string(),
            cost: vec![300],
            rent: vec![26, 130, 390, 900, 1100, 1275],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Property {
            name: "North Carolina Avenue".to_string(),
            cost: vec![300],
            rent: vec![26, 130, 390, 900, 1100, 1275],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Chance("Community Chest".to_string()),
        Tile::Property {
            name: "Pennsylvania Avenue".to_string(),
            cost: vec![320],
            rent: vec![28, 150, 450, 1000, 1200, 1400],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::Railroad,
        Tile::Chance("Chance".to_string()),
        Tile::Property {
            name: "Park Place".to_string(),
            cost: vec![350],
            rent: vec![35, 175, 500, 1100, 1300, 1500],
            level: PropertyLevel::None,
            owner: None,
        },
        Tile::LuxuryTax,
        Tile::Property {
            name: "Boardwalk".to_string(),
            cost: vec![400],
            rent: vec![50, 200, 600, 1400, 1700, 2000],
            level: PropertyLevel::None,
            owner: None,
        },
    ];
}
