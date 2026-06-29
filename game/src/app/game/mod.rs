pub mod init_deck;
pub mod player;
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
            init_deck::init_deck,
            player::Player,
            render::{render_game_banks, render_game_hands},
        },
        render::consts::Direction,
        settings::Settings,
    },
    screens::Screen,
};

pub struct Game<'game> {
    pub deck: Vec<TileData>,
    pub players: HashMap<Wind, Player>,
    pub wind: Wind,
    pub round: usize,
    pub active_seat: Wind,

    bank_size: usize,
    banks: Vec<&'game [TileData]>,
}

impl<'game> Default for Game<'game> {
    fn default() -> Self {
        Self {
            deck: vec![],
            players: HashMap::default(),
            active_seat: Wind::EAST,
            wind: Wind::EAST,
            round: 0,

            bank_size: 0,
            banks: vec![],
        }
    }
}

impl<'game> Game<'game> {
    pub fn init(&mut self, settings: &Settings) {
        self.deck = init_deck(&settings.game);
        self.bank_size = self.deck.len() / 4;

        // TODO: 2x2 dealing hands
        for wind in Wind::iter() {
            let mut player = Player::new(wind.clone(), Direction::DOWN);
            player.add_tiles(
                self.deck
                    .split_off(self.deck.len() - settings.game.hand_size),
            );

            self.players.insert(wind, player);
        }

        self.round = 1;
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
