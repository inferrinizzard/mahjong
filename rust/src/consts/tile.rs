use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

use crate::notation::{
    tile_parse_error::TileParseError, trait_tile_code::INVERSE_TILE_CODE_MAP, TileCode, ToTileCode,
};

#[derive(Debug, Clone)]
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

impl From<TileCode> for TileData {
    fn from(code: TileCode) -> Self {
        TileData::try_from(code.value).unwrap()
    }
}

impl TryFrom<String> for TileData {
    type Error = TileParseError;

    fn try_from(mut tile_string: String) -> Result<Self, Self::Error> {
        let suit_char = tile_string.pop().unwrap();
        let code_char = tile_string.pop().unwrap();

        Ok(match (code_char, suit_char) {
            (c, 'm') => TileData::MAN(TileNumber::from(c)),
            (c, 'p') => TileData::TONG(TileNumber::from(c)),
            (c, 's') => TileData::BAMBOO(TileNumber::from(c)),
            (c, 'z') => {
                let code = c.to_digit(10).unwrap() as usize - 1;

                if code >= 4 {
                    return Ok(TileData::DRAGON(Dragon::iter().nth(code - 4).unwrap()));
                }

                TileData::WIND(Wind::iter().nth(code).unwrap())
            }
            (c, 'f') => {
                let code = c.to_digit(10).unwrap() as usize - 1;

                if code >= 4 {
                    return Ok(TileData::SEASON(Season::iter().nth(code - 4).unwrap()));
                }

                TileData::FLOWER(Flower::iter().nth(code).unwrap())
            }
            _ => {
                return Err(TileParseError {
                    message: format!("Invalid string to parse tile_data: {}", tile_string),
                })
            }
        })
    }
}

impl ToTileCode for TileData {
    fn to_tile_code(&self) -> String {
        match self {
            TileData::MAN(number) => format!("{}m", number.to_string()),
            TileData::TONG(number) => format!("{}p", number.to_string()),
            TileData::BAMBOO(number) => format!("{}s", number.to_string()),
            TileData::WIND(wind) => {
                format!("{}z", Wind::iter().position(|w| &w == wind).unwrap() + 1)
            }
            TileData::DRAGON(dragon) => {
                format!(
                    "{}z",
                    Dragon::iter().position(|d| &d == dragon).unwrap() + 5
                )
            }
            TileData::FLOWER(flower) => {
                format!(
                    "{}f",
                    Flower::iter().position(|f| &f == flower).unwrap() + 1
                )
            }
            TileData::SEASON(season) => {
                format!(
                    "{}f",
                    Season::iter().position(|s| &s == season).unwrap() + 5
                )
            }
        }
    }
}

impl ToString for TileData {
    fn to_string(&self) -> String {
        INVERSE_TILE_CODE_MAP[self.to_tile_code().as_str()].to_string()
    }
}

#[derive(Display, PartialEq, EnumString, EnumIter)]
pub enum Suit {
    MAN,
    TONG,
    BAMBOO,
    WIND,
    DRAGON,
    FLOWER,
    SEASON,
}

impl ToTileCode for Suit {
    fn to_tile_code(&self) -> String {
        match self {
            Suit::MAN => String::from("m"),
            Suit::TONG => String::from("p"),
            Suit::BAMBOO => String::from("s"),
            Suit::WIND => String::from("z"),
            Suit::DRAGON => String::from("z"),
            Suit::FLOWER => String::from("f"),
            Suit::SEASON => String::from("f"),
        }
    }
}

impl Default for Suit {
    fn default() -> Self {
        Suit::MAN
    }
}

#[derive(Debug, Clone, EnumIter)]
pub enum TileNumber {
    ZERO,
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
            TileNumber::ZERO => String::from("0"),
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

impl From<char> for TileNumber {
    fn from(value: char) -> Self {
        match value {
            '0' => TileNumber::ZERO,
            '1' => TileNumber::ONE,
            '2' => TileNumber::TWO,
            '3' => TileNumber::THREE,
            '4' => TileNumber::FOUR,
            '5' => TileNumber::FIVE,
            '6' => TileNumber::SIX,
            '7' => TileNumber::SEVEN,
            '8' => TileNumber::EIGHT,
            '9' => TileNumber::NINE,
            _ => TileNumber::ONE,
        }
    }
}

#[derive(Debug, Display, Clone, PartialEq, EnumString, EnumIter)]
pub enum Wind {
    EAST,
    SOUTH,
    WEST,
    NORTH,
}

#[derive(Debug, Display, Clone, PartialEq, EnumString, EnumIter)]
pub enum Dragon {
    WHITE,
    GREEN,
    RED,
}

#[derive(Debug, Display, Clone, PartialEq, EnumString, EnumIter)]
pub enum Flower {
    PLUM,
    ORCHID,
    CHRYSANTHEMUM,
    BAMBOO,
}

#[derive(Debug, Display, Clone, PartialEq, EnumString, EnumIter)]
pub enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER,
}
