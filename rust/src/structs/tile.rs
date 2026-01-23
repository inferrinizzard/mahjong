use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::Hash,
    str::FromStr,
};

use derivative::Derivative;

use crate::{
    consts::TileData,
    maps::NEXT_TILE_MAP,
    notation::{tile_parse_error::TileParseError, trait_tile_code::TILE_CODE_MAP},
};
use crate::{
    consts::{tile::TileNumber, Dragon, TileName},
    notation::ToTileCode,
};

#[derive(Derivative, Debug)]
#[derivative(Default)]
pub struct Tile {
    pub tile_data: TileData,
    pub name: TileName,

    #[derivative(Default(value = "false"))]
    pub is_wild: bool,
    #[derivative(Default(value = "false"))]
    pub is_dora: bool,
    #[derivative(Default(value = "false"))]
    pub is_akadora: bool,
}

impl Tile {
    pub fn new(tile_data: TileData) -> Tile {
        let name = TileName::from(&tile_data);

        Tile {
            tile_data,
            name,
            ..Default::default()
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(
            self.tile_data,
            TileData::BAMBOO(_) | TileData::MAN(_) | TileData::TONG(_)
        )
    }

    pub fn is_terminal(&self) -> bool {
        self.is_number()
            && match &self.tile_data {
                TileData::BAMBOO(value) | TileData::MAN(value) | TileData::TONG(value) => {
                    matches!(value, TileNumber::ONE | TileNumber::NINE)
                }
                _ => false,
            }
    }

    pub fn is_simple(&self) -> bool {
        self.is_number() && !self.is_terminal()
    }

    pub fn is_honor(&self) -> bool {
        matches!(self.tile_data, TileData::WIND(_) | TileData::DRAGON(_))
    }

    pub fn is_bonus(&self) -> bool {
        matches!(self.tile_data, TileData::SEASON(_) | TileData::FLOWER(_))
    }

    pub fn is_green(&self) -> bool {
        matches!(
            self.tile_data,
            TileData::DRAGON(Dragon::GREEN)
                | TileData::BAMBOO(
                    TileNumber::TWO
                        | TileNumber::THREE
                        | TileNumber::FOUR
                        | TileNumber::SIX
                        | TileNumber::EIGHT
                )
        )
    }
}

impl ToTileCode for Tile {
    fn to_tile_code(&self) -> String {
        if self.is_wild {
            return String::from("0j");
        }

        let tile_code_str = TILE_CODE_MAP[&self.name];

        if self.is_number() && self.is_akadora {
            return String::from("0") + &tile_code_str[1..];
        }

        String::from(tile_code_str)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Tile {
    type Err = TileParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile_data = TileData::try_from(s.to_owned());
        if tile_data.is_err() {
            return Err(tile_data.err().unwrap());
        }

        Ok(Tile::new(tile_data.unwrap()))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Tile {}

impl Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left_index = NEXT_TILE_MAP.get_index_of(&self.name);
        let right_index = NEXT_TILE_MAP.get_index_of(&other.name);

        Some(if left_index > right_index {
            Ordering::Greater
        } else if right_index < left_index {
            Ordering::Less
        } else {
            Ordering::Equal
        })
    }
}
