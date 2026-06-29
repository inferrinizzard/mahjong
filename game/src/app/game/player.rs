use mahjong_lib::{
    consts::Wind,
    tile::{Meld, TileData},
};

use crate::app::render::consts::Direction;

#[derive(Default)]
pub struct Player {
    pub hand: Vec<TileData>,
    pub open_melds: Vec<Meld>,
    pub discard: Vec<TileData>,
    pub active_tile: Option<TileData>,

    pub wind: Wind,
    pub position: Direction,
}

impl Player {
    pub fn new(wind: Wind, position: Direction) -> Self {
        Self {
            hand: vec![],
            open_melds: vec![],
            discard: vec![],
            active_tile: None,

            wind,
            position,
        }
    }

    pub fn add_tiles(&mut self, tiles: Vec<TileData>) {
        self.hand.extend(tiles);
    }
}
