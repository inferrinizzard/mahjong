use crate::consts::Suit;

pub struct Tile {
    suit: Suit,
    value: u16,
    name: String,
}

impl Tile {
    pub fn new(suit: Suit, value: u16) -> Tile {
        let name = value.to_string() + "_" + &suit.to_string();

        Tile { suit, value, name }
    }
}
