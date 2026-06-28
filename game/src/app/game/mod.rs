pub mod hand;
pub mod init_deck;
pub mod render;

use std::collections::HashMap;

use iced::{
    Element,
    widget::{Stack, button, pin},
};
use strum::IntoEnumIterator;

use mahjong_lib::{consts::Wind, tile::TileData};

use crate::{
    app::{
        Message,
        game::{
            hand::GameHand,
            init_deck::init_deck,
            render::{render_game_banks, render_game_hands},
        },
        settings::Settings,
    },
    screens::Screen,
};

pub struct Game {
    pub deck: Vec<TileData>,
    pub hands: HashMap<Wind, GameHand>,
    pub discard: HashMap<Wind, Vec<TileData>>,
    pub turn: usize,
    pub wind: Wind,
    pub round: usize,

    bank_size: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: vec![],
            hands: HashMap::default(),
            discard: HashMap::default(),
            turn: 0,
            wind: Wind::EAST,
            round: 0,

            bank_size: 0,
        }
    }
}

impl Game {
    pub fn init(&mut self, settings: &Settings) {
        self.deck = init_deck(&settings.game);
        self.bank_size = self.deck.len() / 4;

        // TODO: 2x2 dealing hands
        for wind in Wind::iter() {
            self.hands.insert(
                wind.clone(),
                GameHand::from(
                    self.deck
                        .split_off(self.deck.len() - settings.game.hand_size),
                ),
            );
            self.discard.insert(wind, vec![]);
        }

        self.round = 1;
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
        ];

        elements.extend(render_game_banks(&self, settings));
        elements.extend(render_game_hands(&self, settings));

        Stack::from_vec(elements)
            .width(settings.video.window_size.width)
            .height(settings.video.window_size.height)
            .into()
    }
}
