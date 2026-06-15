use std::collections::HashMap;

use crate::consts::TileName;

pub trait ToTileCode {
    fn to_tile_code(&self) -> String;
}

lazy_static! {
    static ref TILE_CODE_LIST: Vec<(TileName, &'static str)> = vec![
        (TileName::BAMBOO_1, "1s"),
        (TileName::BAMBOO_2, "2s"),
        (TileName::BAMBOO_3, "3s"),
        (TileName::BAMBOO_4, "4s"),
        (TileName::BAMBOO_5, "5s"),
        (TileName::BAMBOO_6, "6s"),
        (TileName::BAMBOO_7, "7s"),
        (TileName::BAMBOO_8, "8s"),
        (TileName::BAMBOO_9, "9s"),
        (TileName::MAN_1, "1m"),
        (TileName::MAN_2, "2m"),
        (TileName::MAN_3, "3m"),
        (TileName::MAN_4, "4m"),
        (TileName::MAN_5, "5m"),
        (TileName::MAN_6, "6m"),
        (TileName::MAN_7, "7m"),
        (TileName::MAN_8, "8m"),
        (TileName::MAN_9, "9m"),
        (TileName::TONG_1, "1p"),
        (TileName::TONG_2, "2p"),
        (TileName::TONG_3, "3p"),
        (TileName::TONG_4, "4p"),
        (TileName::TONG_5, "5p"),
        (TileName::TONG_6, "6p"),
        (TileName::TONG_7, "7p"),
        (TileName::TONG_8, "8p"),
        (TileName::TONG_9, "9p"),
        (TileName::WIND_EAST, "1z"),
        (TileName::WIND_SOUTH, "2z"),
        (TileName::WIND_WEST, "3z"),
        (TileName::WIND_NORTH, "4z"),
        (TileName::DRAGON_WHITE, "5z"),
        (TileName::DRAGON_GREEN, "6z"),
        (TileName::DRAGON_RED, "7z"),
        (TileName::FLOWER_PLUM, "1f"),
        (TileName::FLOWER_ORCHID, "2f"),
        (TileName::FLOWER_CHRYSANTHEMUM, "3f"),
        (TileName::FLOWER_BAMBOO, "4f"),
        (TileName::SEASON_SPRING, "5f"),
        (TileName::SEASON_SUMMER, "6f"),
        (TileName::SEASON_AUTUMN, "7f"),
        (TileName::SEASON_WINTER, "8f"),
    ];
    pub static ref TILE_CODE_MAP: HashMap<TileName, &'static str> =
        TILE_CODE_LIST.clone().into_iter().collect();
    pub static ref INVERSE_TILE_CODE_MAP: HashMap<&'static str, TileName> = TILE_CODE_LIST
        .clone()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect();
}
