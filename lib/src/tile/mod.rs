use crate::{consts::consts::Suit, tile::tile_data::TileData};

pub mod tile_data;

pub struct Tile;

impl Tile {
    pub const MAN_1: TileData = TileData {
        suit: Suit::MAN,
        value: 1,
        name: "MAN_1",
        code: "1m",
        index: 1,
    };
}
