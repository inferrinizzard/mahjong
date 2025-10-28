use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
pub enum Suit {
    MAN,
    TONG,
    BAMBOO,
    WIND,
    DRAGON,
    FLOWER,
    SEASON,
}

#[derive(Display, EnumString)]
pub enum Wind {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

#[derive(Display, EnumString)]
pub enum Dragon {
    WHITE = 0,
    GREEN = 1,
    RED = 2,
}

#[derive(Display, EnumString)]
pub enum Flower {
    PLUM,
    LILY,
    CHRYSANTHEMUM,
    BAMBOO,
}

#[derive(Display, EnumString)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}
