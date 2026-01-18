use crate::{consts::Wind, structs::Tile};

#[derive(Debug)]
pub enum MeldType {
    PONG,
    GANG,
    CHI,
}

#[derive(Debug)]
pub struct Meld {
    pub meld_type: MeldType,
    pub tiles: Vec<Tile>,
    pub from: Wind,
}

impl Meld {
    pub fn add_tile(&mut self, fourth_tile: Tile) {
        if fourth_tile != self.tiles[0] {
            panic!("Tried to gang with wrong tile: {}", fourth_tile)
        }

        self.tiles.push(fourth_tile);
    }
}
