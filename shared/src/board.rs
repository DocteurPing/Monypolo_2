use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum PropertyLevel {
    House1,
    House2,
    House3,
    House4,
    Hotel,
}

#[derive(Debug, Clone)]
pub enum Tile {
    Property {
        name: String,
        cost: Vec<u32>,
        rent: Vec<u32>,
        level: PropertyLevel,
        owner: Option<Uuid>,
    },
    Chance(String),
    Jail,
    Go,
    FreeParking,
    Railroad,
    Utility,
    Tax,
    LuxuryTax,
}