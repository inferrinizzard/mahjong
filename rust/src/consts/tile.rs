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
    WHITE,
    GREEN,
    RED,
}

#[derive(Display, EnumString)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}

#[derive(Display, EnumString)]
pub enum Flower {
    PLUM,
    LILY,
    CHRYSANTHEMUM,
    BAMBOO,
}
