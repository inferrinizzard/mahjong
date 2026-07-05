use crate::{TileData, notation::structs::TileString};

pub struct Serializer {}

impl Serializer {
    pub fn serialize_tiles(tiles: Vec<TileData>) -> TileString {
        let tile_codes = tiles.iter().map(|tile| tile.code).collect();
        merge_tile_codes(tile_codes)
    }

    pub fn serialize_tiles_full(tiles: Vec<TileData>) -> TileString {
        let tile_codes: Vec<&str> = tiles.iter().map(|tile| tile.code).collect();
        TileString::from(tile_codes.join(""))
    }
}

fn merge_tile_codes(tile_codes: Vec<&str>) -> TileString {
    let mut tile_string = String::new();
    let mut active_suit = String::new();

    for tile_code in tile_codes {
        let number = tile_code.chars().next().unwrap();
        let suit = tile_code.chars().next().unwrap().to_string();

        if active_suit.is_empty() {
            active_suit = suit.clone();
        }

        if suit != active_suit {
            tile_string.push_str(&suit);
            active_suit = suit.clone();
        }

        tile_string.push(number);
    }
    tile_string.push_str(&active_suit);

    TileString::from(tile_string)
}
