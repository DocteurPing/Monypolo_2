use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum PropertyLevel {
    None = 0,
    House1 = 1,
    House2 = 2,
    House3 = 3,
    House4 = 4,
    Hotel = 5,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Property {
        name: String,
        cost: Vec<u32>,
        rents: Vec<u32>,
        level: PropertyLevel,
        owner: Option<Uuid>,
    },
    Chance(String),
    Jail,
    GoToJail,
    Go {
        amount: u32,
    },
    FreeParking,
    Railroad {
        owner: Option<Uuid>,
        cost: u32,
        rents: Vec<u32>,
    },
    Utility {
        cost: u32,
        owner: Option<Uuid>,
    },
    Tax,
    LuxuryTax,
}
