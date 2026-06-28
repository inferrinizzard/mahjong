use crate::consts::consts::Suit;

#[derive(Debug, Clone)]
pub struct TileData {
    /// Suit of tile
    pub suit: Suit,
    /// index within suit
    pub value: u8,
    /// dev-facing name
    pub name: &'static str,
    /// algorithmic notation code
    pub code: &'static str,
    /// index within standard 34-array
    pub index: i8,
}
