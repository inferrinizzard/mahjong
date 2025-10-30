use std::{cmp::max, collections::HashMap};

use crate::structs::Tile;

type TileFrequencyMap = HashMap<String, u8>;

// consider implementing Deref, DerefMut to expose the map directly
pub struct TileFrequency {
    pub map: TileFrequencyMap,
}

impl TileFrequency {
    fn new() -> TileFrequency {
        TileFrequency {
            map: HashMap::new(),
        }
    }
}

impl From<&Vec<Tile>> for TileFrequency {
    fn from(value: &Vec<Tile>) -> Self {
        let mut map = TileFrequencyMap::new();

        for tile in value {
            *map.entry(tile.name.clone()).or_insert(0) += 1;
        }

        TileFrequency { map }
    }
}
