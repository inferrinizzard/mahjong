use std::ops::{Deref, DerefMut};

use crate::{
    notation::{regex::TILE_CODE_REGEX, structs::TileString},
    tile::TILE_MAP,
};

#[cfg(test)]
#[path = "tile_counts.test.rs"]
mod tests;

type TileCountArray = [usize; 34];

#[derive(Debug, PartialEq)]
pub struct TileCounts {
    value: TileCountArray,
}

impl From<TileString> for TileCounts {
    fn from(value: TileString) -> Self {
        let mut array: TileCountArray = [0; 34];

        TILE_CODE_REGEX
            .find_iter(&value)
            .map(|m| m.as_str())
            .flat_map(|multi_tile_code| {
                let (numbers, suit) = multi_tile_code.split_at(multi_tile_code.len() - 1);
                numbers.chars().map(move |c| format!("{}{}", c, suit))
            })
            .map(|tile_code| TILE_MAP[tile_code.as_str()].index as usize)
            .for_each(|i| array[i] += 1);

        TileCounts { value: array }
    }
}

impl Deref for TileCounts {
    type Target = TileCountArray;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for TileCounts {
    fn deref_mut(&mut self) -> &mut TileCountArray {
        &mut self.value
    }
}
