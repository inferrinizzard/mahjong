pub mod deck;
pub mod init_deck;
pub mod player;
pub mod render;
pub mod tile;
pub use deck::Deck;
pub use player::Player;
pub use tile::GameTile;

use std::collections::HashMap;

use iced::widget::{Stack, button, pin};
use rand::Rng;
use strum::IntoEnumIterator;

use mahjong_lib::consts::Wind;

use crate::{
    app::{
        Component, Message,
        game::render::{render_buttons, render_deck, render_players},
        render::consts::Direction,
        settings::Settings,
    },
    screens::Screen,
};

pub struct Game {
    pub deck: Deck,
    pub players: HashMap<Wind, Player>,
    pub wind: Wind,
    pub round: usize,
    pub active_seat: Wind,
}

#[derive(Debug, Clone)]
pub enum GameMessage {
    SortHand,
    TileClick(String, Direction),
}

impl Default for Game {
    fn default() -> Self {
        Self {
            deck: Deck::default(),
            players: HashMap::default(),
            active_seat: Wind::EAST,
            wind: Wind::EAST,
            round: 0,
        }
    }
}

impl Game {
    pub fn init(&mut self, settings: &Settings) {
        self.reset_round(settings);
    }

    pub fn reset_round(&mut self, settings: &Settings) {
        self.deck = Deck::new(&settings.game);

        let mut rng = rand::rng();
        let dice_roll: usize = rng.random_range(1..=6) + rng.random_range(1..=6);
        self.deck.init_index(dice_roll);

        // TODO: 2x2 dealing hands
        for wind in Wind::iter() {
            let mut player = Player::new(wind.clone(), Direction::DOWN);
            player.add_tiles(self.deck.draw_tiles(settings.game.hand_size));

            self.players.insert(wind, player);
        }

        self.round += 1;
    }

    pub fn update(&mut self, _settings: &Settings, message: GameMessage) {
        match message {
            GameMessage::SortHand => {
                self.players.get_mut(&Wind::EAST).unwrap().hand.sort();
            }
            GameMessage::TileClick(id, direction) => {
                println!("{}, {:?}", id, direction);
            }
        }
    }

    pub fn view(&self, settings: &Settings) -> Component {
        let mut elements = vec![
            pin(button("Back to Main").on_press(Message::ChangeScreen(Screen::Main)))
                .x(0)
                .y(0)
                .into(),
        ];

        elements.extend(render_deck(&self.deck, settings));
        elements.extend(render_players(&self.players, settings));
        elements.extend(render_buttons(settings));

        Stack::from_vec(elements)
            .width(settings.video.window_size.width)
            .height(settings.video.window_size.height)
            .into()
    }
}
