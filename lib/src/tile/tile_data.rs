use crate::consts::consts::Suit;

#[derive(Debug, Clone, Eq, Ord)]
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

impl PartialEq for TileData {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.value == other.value
    }
}

impl PartialOrd for TileData {
    fn gt(&self, other: &Self) -> bool {
        self.suit > other.suit || (self.suit == other.suit && self.value > other.value)
    }

    fn ge(&self, other: &Self) -> bool {
        TileData::gt(self, other) || self == other
    }

    fn lt(&self, other: &Self) -> bool {
        self.suit < other.suit || (self.suit == other.suit && self.value < other.value)
    }

    fn le(&self, other: &Self) -> bool {
        TileData::lt(self, other) || self == other
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self > other {
            Some(std::cmp::Ordering::Greater)
        } else if self < other {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}
