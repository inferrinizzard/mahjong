use std::{cmp::max, collections::HashMap};

use crate::{consts::TileName, structs::Tile};

type TileFrequencyMap = HashMap<TileName, u8>;

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

    pub fn increment(&mut self, key: &TileName) {
        let mut new_count = self.map.get(key).unwrap_or(&0) + 1;
        self.map.get_mut(key).insert(&mut new_count);
    }
    pub fn decrement(&mut self, key: &TileName) {
        let mut new_count = max(self.map.get(key).unwrap_or(&0) - 1, 0);
        self.map.get_mut(key).insert(&mut new_count);
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
