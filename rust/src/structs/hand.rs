use crate::{maps::TileFrequency, structs::Tile};

pub struct Hand {
    pub tiles: Vec<Tile>,
    tile_frequency: TileFrequency,
}

impl Hand {
    pub fn new(tiles: Vec<Tile>) -> Hand {
        let tile_frequency = TileFrequency::from(&tiles);
        Hand {
            tiles,
            tile_frequency,
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tile_frequency.increment(&tile.name);
        self.tiles.push(tile);
    }
    pub fn remove_tile(&mut self, tile: Tile) {
        self.tile_frequency.decrement(&tile.name);
        // self.tiles.push(tile);
    }
}
