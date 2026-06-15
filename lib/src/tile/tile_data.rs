use crate::consts::consts::Suit;

#[derive(Debug, Clone)]
pub struct TileData {
    pub suit: Suit,
    pub value: u8,
    pub name: &'static str,
    pub code: &'static str,
    pub index: i8,
}
