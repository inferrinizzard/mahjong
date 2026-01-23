use strum::IntoEnumIterator;

use crate::{
    consts::{Suit, TileData},
    maps::TileFrequency,
    notation::{trait_tile_code::TILE_CODE_MAP, ToTileCode},
};

pub struct Serializer {}

impl Serializer {
    pub fn serialize_tiles(tiles: Vec<TileData>) -> String {
        let tile_codes = tiles.iter().map(|tile| tile.to_tile_code()).collect();
        merge_tile_code_list(tile_codes)
    }

    pub fn serialize_tile_frequency(tile_frequency: TileFrequency) -> String {
        let mut tile_codes = vec![];

        tile_frequency.map.iter().for_each(|(tile_name, count)| {
            let tile_code = TILE_CODE_MAP[tile_name];
            for _ in 0..*count {
                tile_codes.push(tile_code);
            }
        });

        merge_tile_code_list_str(tile_codes)
    }
}

fn merge_tile_code_list(tile_codes: Vec<String>) -> String {
    merge_tile_code_list_str(tile_codes.iter().map(|tile| tile.as_str()).collect())
}
fn merge_tile_code_list_str(tile_codes: Vec<&str>) -> String {
    let mut tile_string = String::new();

    Suit::iter().for_each(|suit| {
        // DRAGON and SEASON share same code as WIND and FLOWER, skip double process
        if suit == Suit::DRAGON || suit == Suit::SEASON {
            return;
        }

        let suit_code = suit.to_tile_code();
        let tiles_of_suit = tile_codes.iter().filter(|tile| tile.ends_with(&suit_code));
        let mut sorted_tile_values = tiles_of_suit
            .map(|tile| tile.chars().nth(0).unwrap())
            .collect::<Vec<char>>();
        sorted_tile_values.sort();
        sorted_tile_values.iter().for_each(|c| tile_string.push(*c));

        if sorted_tile_values.len() > 0 {
            tile_string.push_str(suit_code.as_str());
        }
    });

    tile_string
}
