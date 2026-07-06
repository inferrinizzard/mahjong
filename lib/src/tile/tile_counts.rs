use std::ops::{Deref, DerefMut};

use crate::{
    notation::{regex::TILE_CODE_REGEX, structs::TileString},
    tile::TILE_MAP,
};

type TileCountArray = [usize; 34];

pub struct TileCounts {
    value: TileCountArray,
}

impl From<TileString> for TileCounts {
    fn from(value: TileString) -> Self {
        let mut array: TileCountArray = [0; 34];

        TILE_CODE_REGEX
            .find_iter(&value)
            .map(|m| m.as_str())
            .map(|tile_code| TILE_MAP[tile_code].index as usize)
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
