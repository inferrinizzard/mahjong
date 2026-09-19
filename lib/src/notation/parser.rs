use crate::{
    TileData,
    notation::regex::TILE_CODE_REGEX,
    tile::{TILE_MAP, TileCounts},
    types::TileCode,
};

use super::tile_parse_error::TileParseError;

pub struct Parser {}

impl Parser {
    fn parse_to_tile_codes(input: impl AsRef<str>) -> Result<Vec<TileCode>, TileParseError> {
        let tile_matches = TILE_CODE_REGEX
            .find_iter(input.as_ref())
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        let tile_codes = tile_matches
            .iter()
            .flat_map(|s| split_tile_codes(s))
            .collect();

        Ok(tile_codes)
    }

    /// Parses input str, String, or TileString
    /// Skips invalid input text and only returns matching tile codes
    /// Returns Vec of full tile data for given input
    pub fn parse(input: impl AsRef<str>) -> Result<Vec<TileData>, TileParseError> {
        let result = Parser::parse_to_tile_codes(input);

        if let Ok(tile_codes) = result {
            let tiles: Vec<TileData> = tile_codes
                .iter()
                .map(|s| TILE_MAP[s.as_str()].clone())
                .collect();

            return Ok(tiles);
        }

        Err(result.unwrap_err())
    }

    /// Parses input str, String, or TileString
    /// Skips invalid input text and only returns matching tile codes
    /// Returns tile count array of 34 length
    pub fn parse_to_counts(input: impl AsRef<str>) -> Result<TileCounts, TileParseError> {
        let result = Parser::parse_to_tile_codes(input);

        if let Ok(tile_codes) = result {
            return Ok(TileCounts::from(tile_codes));
        }
        Err(result.unwrap_err())
    }
}

fn split_tile_codes(s: &str) -> Vec<String> {
    let (tile_numbers, suit) = s.split_at(s.len() - 1);

    tile_numbers
        .chars()
        .map(|c| format!("{}{}", c, suit))
        .collect()
}
