use std::str::FromStr;

use strum_macros::{Display, EnumString};

use crate::consts::{Suit, TileData};

#[allow(non_camel_case_types)]
#[derive(Debug, Display, Clone, PartialEq, Eq, Hash, EnumString)]
pub enum TileName {
    MAN_1,
    MAN_2,
    MAN_3,
    MAN_4,
    MAN_5,
    MAN_6,
    MAN_7,
    MAN_8,
    MAN_9,
    TONG_1,
    TONG_2,
    TONG_3,
    TONG_4,
    TONG_5,
    TONG_6,
    TONG_7,
    TONG_8,
    TONG_9,
    BAMBOO_1,
    BAMBOO_2,
    BAMBOO_3,
    BAMBOO_4,
    BAMBOO_5,
    BAMBOO_6,
    BAMBOO_7,
    BAMBOO_8,
    BAMBOO_9,
    WIND_EAST,
    WIND_SOUTH,
    WIND_WEST,
    WIND_NORTH,
    DRAGON_WHITE,
    DRAGON_GREEN,
    DRAGON_RED,
    FLOWER_PLUM,
    FLOWER_ORCHID,
    FLOWER_CHRYSANTHEMUM,
    FLOWER_BAMBOO,
    SEASON_SPRING,
    SEASON_SUMMER,
    SEASON_AUTUMN,
    SEASON_WINTER,
}

impl From<&TileData> for TileName {
    fn from(tile_data: &TileData) -> Self {
        let (suit, value) = match tile_data {
            TileData::MAN(num) => (Suit::MAN, num.to_string()),
            TileData::TONG(num) => (Suit::TONG, num.to_string()),
            TileData::BAMBOO(num) => (Suit::BAMBOO, num.to_string()),
            TileData::WIND(wind) => (Suit::WIND, wind.to_string()),
            TileData::DRAGON(dragon) => (Suit::DRAGON, dragon.to_string()),
            TileData::FLOWER(flower) => (Suit::FLOWER, flower.to_string()),
            TileData::SEASON(season) => (Suit::SEASON, season.to_string()),
        };
        let tile_name_str = format!("{}_{}", suit, value);

        TileName::from_str(&tile_name_str.as_str()).unwrap()
    }
}

impl Default for TileName {
    fn default() -> Self {
        TileName::from(&TileData::default())
    }
}
