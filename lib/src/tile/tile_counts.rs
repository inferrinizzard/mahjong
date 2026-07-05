use std::ops::{Deref, DerefMut};

// use crate::notation::structs::TileString;TileCounts

type TileCountArray = [usize; 34];

pub struct TileCounts {
    value: TileCountArray,
}

// impl TileCounts {
//     pub fn new(tile_string: TileString) -> Self {
//         // tile_string.
//     }
// }

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
