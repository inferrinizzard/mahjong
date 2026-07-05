use crate::{TileData, notation::regex::TILE_CODE_REGEX, tile::TILE_MAP};

use super::tile_parse_error::TileParseError;

pub struct Parser {}

impl Parser {
    pub fn parse_str(s: &str) -> Result<Vec<TileData>, TileParseError> {
        let tile_matches = TILE_CODE_REGEX
            .find_iter(s)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        let tiles: Vec<TileData> = tile_matches
            .iter()
            .flat_map(|s| split_tile_codes(s))
            .map(|s| TILE_MAP[s.as_str()].clone())
            .collect();

        Ok(tiles)
    }
}

fn split_tile_codes(s: &str) -> Vec<String> {
    let (tile_numbers, suit) = s.split_at(s.len() - 1);

    tile_numbers
        .chars()
        .map(|c| format!("{}{}", c, suit))
        .collect()
}
