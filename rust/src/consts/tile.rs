use strum_macros::{Display, EnumIter, EnumString};

pub enum TileData {
    MAN(TileNumber),
    TONG(TileNumber),
    BAMBOO(TileNumber),
    WIND(Wind),
    DRAGON(Dragon),
    FLOWER(Flower),
    SEASON(Season),
}

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

impl Default for Suit {
    fn default() -> Self {
        Suit::MAN
    }
}

#[derive(Display, EnumString, EnumIter)]
pub enum TileNumber {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
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
