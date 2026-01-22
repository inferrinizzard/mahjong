use regex::Regex;

use super::tile_parse_error::TileParseError;
use crate::consts::TileData;

lazy_static! {
    static ref TILE_REGEX: Regex = Regex::new(r"\d+\w").unwrap();
}

pub struct Parser {}

impl Parser {
    pub fn parse_string(s: &str) -> Result<Vec<TileData>, TileParseError> {
        let tile_matches = TILE_REGEX
            .find_iter(s)
            .map(|m| m.as_str())
            .collect::<Vec<&str>>();

        let tiles = tile_matches
            .iter()
            .map(|s| parse_tiles_for_suit(s).unwrap())
            .collect::<Vec<Vec<TileData>>>();

        Ok(tiles.iter().flatten().collect::<Vec<TileData>>())
    }
}

fn parse_tiles_for_suit(s: &str) -> Result<Vec<TileData>, TileParseError> {
    let mut input_tile_string = String::from(s);
    let suit_char = input_tile_string.pop().unwrap();
    // let suit =
    //  {
    //     Some(c) => c,
    //     None => return Err(TileParseError::new("Missing a suit")),
    // };
    let tile_numbers = input_tile_string;
    let tile_number_strings = tile_numbers.chars().map(|c| format!("{}{}", c, suit_char));
    let tiles = tile_number_strings
        .map(|tile_string| TileData::try_from(tile_string).unwrap())
        .collect::<Vec<TileData>>();

    Ok(tiles)
}
