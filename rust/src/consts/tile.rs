use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug)]
pub enum TileData {
    MAN(TileNumber),
    TONG(TileNumber),
    BAMBOO(TileNumber),
    WIND(Wind),
    DRAGON(Dragon),
    FLOWER(Flower),
    SEASON(Season),
}

impl Default for TileData {
    fn default() -> Self {
        TileData::DRAGON(Dragon::RED)
    }
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

#[derive(Debug, EnumIter)]
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

impl ToString for TileNumber {
    fn to_string(&self) -> String {
        match self {
            TileNumber::ONE => String::from("1"),
            TileNumber::TWO => String::from("2"),
            TileNumber::THREE => String::from("3"),
            TileNumber::FOUR => String::from("4"),
            TileNumber::FIVE => String::from("5"),
            TileNumber::SIX => String::from("6"),
            TileNumber::SEVEN => String::from("7"),
            TileNumber::EIGHT => String::from("8"),
            TileNumber::NINE => String::from("9"),
        }
    }
}

#[derive(Debug, Display, EnumString, EnumIter)]
pub enum Wind {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

#[derive(Debug, Display, EnumString)]
pub enum Dragon {
    WHITE,
    GREEN,
    RED,
}

#[derive(Debug, Display, EnumString)]
pub enum Flower {
    PLUM,
    LILY,
    CHRYSANTHEMUM,
    BAMBOO,
}

#[derive(Debug, Display, EnumString)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}
