use crate::{
    Tile, TileData,
    notation::{parser_util::ParserUtil, regex::TILE_CODE_REGEX},
    tile::TileCounts,
    types::TileCode,
};

use super::tile_parse_error::TileParseError;

pub struct Parser {}

impl Parser {
    pub fn parse_to_tile_codes(input: impl AsRef<str>) -> Result<Vec<TileCode>, TileParseError> {
        let tile_matches = TILE_CODE_REGEX
            .find_iter(input.as_ref())
            .map(|m| m.as_str());

        let tile_codes = tile_matches
            .flat_map(|s| ParserUtil::split_tile_codes(s))
            .collect();

        Ok(tile_codes)
    }

    /// Parses input str, String, or TileString
    /// Skips invalid input text and only returns matching tile codes
    /// Returns Vec of full tile data for given input
    pub fn parse(input: impl AsRef<str>) -> Result<Vec<TileData>, TileParseError> {
        let result = Parser::parse_to_tile_codes(input);

        if let Ok(tile_codes) = result {
            let tiles: Vec<TileData> = tile_codes.iter().map(Tile::get_tile_from_code).collect();

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

    /// Checks if input matchs mpsz format
    pub fn is_valid(input: impl AsRef<str>) -> bool {
        TILE_CODE_REGEX.is_match(input.as_ref())
    }
}
