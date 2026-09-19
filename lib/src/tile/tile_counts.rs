use std::ops::{Deref, DerefMut};

use crate::{
    Tile,
    notation::{Parser, structs::TileString},
    types::TileCode,
};

#[cfg(test)]
#[path = "tile_counts__test.rs"]
mod unit_test;

type TileCountArray = [usize; 34];

#[derive(Debug, PartialEq)]
pub struct TileCounts {
    value: TileCountArray,
}

impl From<Vec<TileCode>> for TileCounts {
    fn from(value: Vec<TileCode>) -> Self {
        let mut array: TileCountArray = [0; 34];

        value
            .iter()
            .map(Tile::get_tile_from_code)
            .map(|tile| tile.index as usize)
            .for_each(|i| array[i] += 1);

        TileCounts { value: array }
    }
}

// impl TryFrom<TileString> for TileCounts {}
// TODO: throw if >4 count

impl From<TileString> for TileCounts {
    fn from(value: TileString) -> Self {
        let tile_codes = Parser::parse_to_tile_codes(value).unwrap();

        Self::from(tile_codes)
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
