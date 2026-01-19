use derivative::Derivative;

use crate::{
    maps::TileFrequency,
    structs::{meld::Meld, Tile},
};

#[derive(Debug, Derivative)]
#[derivative(Default)]
pub struct Hand {
    pub tiles: Vec<Tile>,
    pub tile_frequency: TileFrequency,
    pub melds: Vec<Meld>,

    #[derivative(Default(value = "0"))]
    pub num_wilds: u8,

    #[derivative(Default(value = "false"))]
    pub is_riichi: bool,
}

impl Hand {
    pub fn new(tiles: Vec<Tile>) -> Hand {
        let tile_frequency = TileFrequency::from(&tiles);
        Hand {
            tiles,
            tile_frequency,
            ..Default::default()
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        if tile.is_wild {
            self.num_wilds += 1;
        } else {
            self.tile_frequency.increment(&tile.name);
        }
        self.tiles.push(tile);
    }
    pub fn remove_tile(&mut self, tile: Tile) {
        if tile.is_wild {
            self.num_wilds -= 1;
        } else {
            self.tile_frequency.decrement(&tile.name);
        }

        // self.tiles
    }

    // pub fn get_possible_melds(&self, &tile: Tile) {}
}
