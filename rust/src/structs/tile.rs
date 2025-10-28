use std::{
    cmp::Ordering,
    fmt::{self, Display},
    num::ParseIntError,
    str::FromStr,
};

use derivative::Derivative;
use strum::ParseError;

use crate::consts::Suit;
use crate::maps::next_tile_map;

#[derive(Derivative)]
#[derivative(Default)]
pub struct Tile {
    pub suit: Suit,
    pub value: u8,
    pub name: String,

    #[derivative(Default(value = "false"))]
    pub is_wild: bool,
    #[derivative(Default(value = "false"))]
    pub is_dora: bool,
    #[derivative(Default(value = "false"))]
    pub is_akadora: bool,
}

impl Tile {
    pub fn new(suit: Suit, value: u8) -> Tile {
        let name = value.to_string() + "_" + &suit.to_string();

        Tile {
            suit,
            value,
            name,
            ..Default::default()
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self.suit, Suit::BAMBOO | Suit::MAN | Suit::TONG)
    }

    pub fn is_terminal(&self) -> bool {
        self.is_number() && matches!(self.value, 1 | 9)
    }

    pub fn is_simple(&self) -> bool {
        self.is_number() && !self.is_terminal()
    }

    pub fn is_honor(&self) -> bool {
        matches!(self.suit, Suit::WIND | Suit::DRAGON)
    }

    pub fn is_bonus(&self) -> bool {
        matches!(self.suit, Suit::SEASON | Suit::FLOWER)
    }

    pub fn is_green(&self) -> bool {
        match self {
            Tile {
                suit: Suit::DRAGON,
                value: 1,
                ..
            } => true,
            Tile {
                suit: Suit::BAMBOO,
                value: 2 | 3 | 4 | 6 | 8,
                ..
            } => true,
            _ => false,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct TileParseError {
    message: String,
}
impl From<ParseError> for TileParseError {
    fn from(value: ParseError) -> Self {
        TileParseError {
            message: value.to_string(),
        }
    }
}
impl From<ParseIntError> for TileParseError {
    fn from(value: ParseIntError) -> Self {
        TileParseError {
            message: value.to_string(),
        }
    }
}

impl FromStr for Tile {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slugs = s.split("_").take(2).collect::<Vec<_>>();
        let value = u8::from_str(slugs[0])?;
        let suit = Suit::from_str(slugs[1])?;

        Ok(Tile {
            suit,
            value,
            name: s.to_string(),
            ..Default::default()
        })
    }
    type Err = TileParseError;
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left_index = next_tile_map.get_index_of(self.name.as_str());
        let right_index = next_tile_map.get_index_of(other.name.as_str());

        Some(if left_index > right_index {
            Ordering::Greater
        } else if right_index < left_index {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }
}
