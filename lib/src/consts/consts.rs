use strum_macros::{EnumIter, EnumString};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, EnumString, EnumIter)]
pub enum Wind {
    #[default]
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

#[derive(Debug)]
pub enum Dragon {
    WHITE,
    GREEN,
    RED,
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
