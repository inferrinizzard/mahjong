use std::collections::HashMap;

use mahjong_lib::{consts::Wind, tile::TileData};

use crate::app::init_deck::init_deck;

type Hand = Vec<TileData>;

pub struct Game {
    pub deck: Vec<TileData>,
    pub hands: HashMap<u8, Hand>,
    pub discard: HashMap<u8, Vec<TileData>>,
    pub turn: u8,
    pub wind: Wind,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: vec![],
            hands: [].into_iter().collect(),
            discard: [].into_iter().collect(),
            turn: 0,
            wind: Wind::EAST,
        }
    }
}

impl Game {
    pub fn init(&mut self) {
        self.deck = init_deck();

        // TODO: hand_size settings
        let hand_size = 13;
        // TODO: 2x2 dealing hands
        for i in 0..4 {
            self.hands
                .insert(i, self.deck.split_off(self.deck.len() - hand_size));
            self.discard.insert(i, vec![]);
        }

        self.turn = 1;
    }
}
