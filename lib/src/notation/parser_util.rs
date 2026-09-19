use crate::types::TileCode;

pub struct ParserUtil {}

impl ParserUtil {
    /// Splits a str of format 123456789m
    /// into a vec of TileCode: [1m, 2m, ... 9m]
    pub fn split_tile_codes(s: &str) -> Vec<TileCode> {
        let (tile_numbers, suit) = s.split_at(s.len() - 1);

        tile_numbers
            .chars()
            .map(|c| format!("{}{}", c, suit))
            .collect()
    }
}
