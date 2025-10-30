use std::collections::HashMap;

use crate::{
    traits::parse_tile_code,
    types::{TileCode, TileName},
};

type TileFrequencyMap = HashMap<TileName, u8>;

// consider implementing Deref, DerefMut to expose the map directly
pub struct TileFrequency {
    map: TileFrequencyMap,
}

impl TileFrequency {
    fn new() -> TileFrequency {
        TileFrequency {
            map: HashMap::new(),
        }
    }
}

impl From<Vec<TileName>> for TileFrequency {
    fn from(value: Vec<TileName>) -> Self {
        let mut map = TileFrequencyMap::new();

        for tile_name in value {
            *map.entry(tile_name).or_insert(0) += 1;
        }

        TileFrequency { map }
    }
}

impl From<TileCode> for TileFrequency {
    fn from(value: TileCode) -> Self {
        TileFrequency::from(parse_tile_code(value))
    }
}
