pub mod init_deck;
pub mod render;

use std::collections::HashMap;

use iced::{
    Element,
    widget::{Stack, button, pin, stack},
};
use mahjong_lib::{consts::Wind, tile::TileData};

use crate::{
    app::{
        Message,
        game::{init_deck::init_deck, render::render_game_hands},
        render::render_tile::{render_bank, render_hand},
        settings::Settings,
    },
    screens::Screen,
};

type Hand = Vec<TileData>;

pub struct Game {
    pub deck: Vec<TileData>,
    pub hands: HashMap<u8, Hand>,
    // pub open_tiles: HashMap<u8, Hand>,
    pub discard: HashMap<u8, Vec<TileData>>,
    pub turn: u8,
    pub wind: Wind,

    bank_size: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: vec![],
            hands: [].into_iter().collect(),
            discard: [].into_iter().collect(),
            turn: 0,
            wind: Wind::EAST,

            bank_size: 0,
        }
    }
}

impl Game {
    pub fn init(&mut self) {
        self.deck = init_deck();
        self.bank_size = self.deck.len() / 4;

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

    pub fn view(&self, settings: &Settings) -> Element<'static, Message> {
        // render hands 1,2,3,4
        // render banks 1,2,3,4
        // render discards 1,2,3,4
        // render open_tiles 1,2,3,4
        // render ui

        let mut elements = vec![
            pin(button("Back to Main").on_press(Message::ChangeScreen(Screen::Main)))
                .x(0)
                .y(0)
                .into(),
            pin(render_bank(self.bank_size, settings.video.tile_size))
                .x(0)
                .y(128)
                .into(),
        ];

        elements.extend(render_game_hands(&self, settings));

        Stack::from_vec(elements)
            .width(settings.video.window_size.width)
            .height(settings.video.window_size.height)
            .into()
    }
}
