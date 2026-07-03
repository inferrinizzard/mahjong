use mahjong_lib::{consts::Wind, tile::Meld};

use crate::app::{game::GameTile, render::consts::Direction};

#[derive(Default)]
pub struct Player {
    pub hand: Vec<GameTile>,
    pub open_melds: Vec<Meld>,
    pub discard: Vec<GameTile>,
    pub active_tile: Option<GameTile>,

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

    pub fn add_tiles(&mut self, tiles: Vec<GameTile>) {
        self.hand.extend(tiles);
    }
}
