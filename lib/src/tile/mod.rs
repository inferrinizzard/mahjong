use crate::{consts::consts::Suit, tile::tile_data::TileData};

pub mod tile_data;

pub struct Tile;

impl Tile {
    pub const MAN_1: TileData = TileData {
        suit: Suit::MAN,
        value: 1,
        name: "MAN_1",
        code: "1m",
        index: 0,
    };
    pub const MAN_2: TileData = TileData {
        suit: Suit::MAN,
        value: 2,
        name: "MAN_2",
        code: "2m",
        index: 2,
    };
    pub const MAN_3: TileData = TileData {
        suit: Suit::MAN,
        value: 3,
        name: "MAN_3",
        code: "3m",
        index: 2,
    };
    pub const MAN_4: TileData = TileData {
        suit: Suit::MAN,
        value: 4,
        name: "MAN_4",
        code: "4m",
        index: 3,
    };
    pub const MAN_5: TileData = TileData {
        suit: Suit::MAN,
        value: 5,
        name: "MAN_5",
        code: "5m",
        index: 4,
    };
    pub const MAN_6: TileData = TileData {
        suit: Suit::MAN,
        value: 6,
        name: "MAN_6",
        code: "6m",
        index: 5,
    };
    pub const MAN_7: TileData = TileData {
        suit: Suit::MAN,
        value: 7,
        name: "MAN_7",
        code: "7m",
        index: 6,
    };
    pub const MAN_8: TileData = TileData {
        suit: Suit::MAN,
        value: 8,
        name: "MAN_8",
        code: "8m",
        index: 7,
    };
    pub const MAN_9: TileData = TileData {
        suit: Suit::MAN,
        value: 9,
        name: "MAN_9",
        code: "9m",
        index: 8,
    };

    pub const TONG_1: TileData = TileData {
        suit: Suit::TONG,
        value: 1,
        name: "TONG_1",
        code: "1p",
        index: 9,
    };
    pub const TONG_2: TileData = TileData {
        suit: Suit::TONG,
        value: 2,
        name: "TONG_2",
        code: "2p",
        index: 10,
    };
    pub const TONG_3: TileData = TileData {
        suit: Suit::TONG,
        value: 3,
        name: "TONG_3",
        code: "3p",
        index: 11,
    };
    pub const TONG_4: TileData = TileData {
        suit: Suit::TONG,
        value: 4,
        name: "TONG_4",
        code: "4p",
        index: 12,
    };
    pub const TONG_5: TileData = TileData {
        suit: Suit::TONG,
        value: 5,
        name: "TONG_5",
        code: "5p",
        index: 13,
    };
    pub const TONG_6: TileData = TileData {
        suit: Suit::TONG,
        value: 6,
        name: "TONG_6",
        code: "6p",
        index: 14,
    };
    pub const TONG_7: TileData = TileData {
        suit: Suit::TONG,
        value: 7,
        name: "TONG_7",
        code: "7p",
        index: 15,
    };
    pub const TONG_8: TileData = TileData {
        suit: Suit::TONG,
        value: 8,
        name: "TONG_8",
        code: "8p",
        index: 16,
    };
    pub const TONG_9: TileData = TileData {
        suit: Suit::TONG,
        value: 9,
        name: "TONG_9",
        code: "9p",
        index: 17,
    };

    pub const BAMBOO_1: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 1,
        name: "BAMBOO_1",
        code: "1s",
        index: 18,
    };
    pub const BAMBOO_2: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 2,
        name: "BAMBOO_2",
        code: "2s",
        index: 19,
    };
    pub const BAMBOO_3: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 3,
        name: "BAMBOO_3",
        code: "3s",
        index: 20,
    };
    pub const BAMBOO_4: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 4,
        name: "BAMBOO_4",
        code: "4s",
        index: 21,
    };
    pub const BAMBOO_5: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 5,
        name: "BAMBOO_5",
        code: "5s",
        index: 22,
    };
    pub const BAMBOO_6: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 6,
        name: "BAMBOO_6",
        code: "6s",
        index: 23,
    };
    pub const BAMBOO_7: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 7,
        name: "BAMBOO_7",
        code: "7s",
        index: 24,
    };
    pub const BAMBOO_8: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 8,
        name: "BAMBOO_8",
        code: "8s",
        index: 25,
    };
    pub const BAMBOO_9: TileData = TileData {
        suit: Suit::BAMBOO,
        value: 9,
        name: "BAMBOO_9",
        code: "9s",
        index: 26,
    };

    pub const EAST_WIND: TileData = TileData {
        suit: Suit::WIND,
        value: 1,
        name: "EAST_WIND",
        code: "1z",
        index: 27,
    };
    pub const SOUTH_WIND: TileData = TileData {
        suit: Suit::WIND,
        value: 2,
        name: "SOUTH_WIND",
        code: "2z",
        index: 28,
    };
    pub const WEST_WIND: TileData = TileData {
        suit: Suit::WIND,
        value: 3,
        name: "WEST_WIND",
        code: "3z",
        index: 29,
    };
    pub const NORTH_WIND: TileData = TileData {
        suit: Suit::WIND,
        value: 4,
        name: "NORTH_WIND",
        code: "4z",
        index: 30,
    };

    pub const WHITE_DRAGON: TileData = TileData {
        suit: Suit::DRAGON,
        value: 1,
        name: "WHITE_DRAGON",
        code: "5z",
        index: 31,
    };
    pub const GREEN_GRADON: TileData = TileData {
        suit: Suit::DRAGON,
        value: 2,
        name: "GREEN_GRADON",
        code: "6z",
        index: 31,
    };
    pub const RED_DRAGON: TileData = TileData {
        suit: Suit::DRAGON,
        value: 3,
        name: "RED_DRAGON",
        code: "7z",
        index: 32,
    };

    pub const PLUM_FLOWER: TileData = TileData {
        suit: Suit::FLOWER,
        value: 1,
        name: "PLUM_FLOWER",
        code: "1f",
        index: 33,
    };
    pub const LILY_FLOWER: TileData = TileData {
        suit: Suit::FLOWER,
        value: 2,
        name: "LILY_FLOWER",
        code: "2f",
        index: 34,
    };
    pub const CHRYSANTHEMUM_FLOWER: TileData = TileData {
        suit: Suit::FLOWER,
        value: 3,
        name: "CHRYSANTHEMUM_FLOWER",
        code: "3f",
        index: 35,
    };
    pub const BAMBOO_FLOWER: TileData = TileData {
        suit: Suit::FLOWER,
        value: 4,
        name: "BAMBOO_FLOWER",
        code: "4f",
        index: 36,
    };

    pub const SPRING_SEASON: TileData = TileData {
        suit: Suit::SEASON,
        value: 1,
        name: "SPRING_SEASON",
        code: "5f",
        index: 37,
    };
    pub const SUMMER_SEASON: TileData = TileData {
        suit: Suit::SEASON,
        value: 2,
        name: "SUMMER_SEASON",
        code: "6f",
        index: 38,
    };
    pub const AUTUMN_SEASON: TileData = TileData {
        suit: Suit::SEASON,
        value: 3,
        name: "AUTUMN_SEASON",
        code: "7f",
        index: 39,
    };
    pub const WINTER_SEASON: TileData = TileData {
        suit: Suit::SEASON,
        value: 4,
        name: "WINTER_SEASON",
        code: "8f",
        index: 40,
    };

    pub const JOKER: TileData = TileData {
        suit: Suit::JOKER,
        value: 1,
        name: "JOKER",
        code: "1j",
        index: 41,
    };
}
