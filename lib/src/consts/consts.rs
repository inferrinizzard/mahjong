use strum::EnumCount;
use strum_macros::{EnumCount, FromRepr};

use crate::traits::Next;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    MAN,
    TONG,
    BAMBOO,
    WIND,
    DRAGON,
    FLOWER,
    SEASON,
    JOKER,
}

#[derive(Debug, Clone, Default, EnumCount, FromRepr)]
pub enum Wind {
    #[default]
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

impl Next for Wind {
    fn next(&self) -> Self {
        Wind::from_repr((self.clone() as usize + 1) % Wind::COUNT).unwrap()
    }
}

#[derive(Debug, Clone, EnumCount, FromRepr)]
pub enum Dragon {
    WHITE,
    GREEN,
    RED,
}

impl Next for Dragon {
    fn next(&self) -> Self {
        Dragon::from_repr((self.clone() as usize + 1) % Dragon::COUNT).unwrap()
    }
}

#[derive(Debug)]
pub enum Flower {
    PLUM,
    ORCHID,
    CHRYSANTHEMUM,
    BAMBOO,
}

#[derive(Debug)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}
