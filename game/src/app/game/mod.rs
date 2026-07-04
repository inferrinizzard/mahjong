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

use mahjong_lib::{consts::Wind, traits::Next};

use crate::{
    app::{
        Component, Message,
        game::render::{render_buttons, render_deck},
        render::consts::Direction,
        settings::Settings,
    },
    screens::Screen,
};

pub struct Game {
    pub deck: Deck,
    pub players: HashMap<Direction, Player>,
    pub wind: Wind,
    pub round: usize,
    pub active_seat: Direction,
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
            active_seat: Direction::DOWN,
            wind: Wind::EAST,
            round: 0,
        }
    }
}

impl Game {
    pub fn init(&mut self, settings: &Settings) {
        self.reset_round(settings);

        self.draw_tile();
    }

    pub fn draw_tile(&mut self) {
        let tile = self.deck.draw_tiles(1).pop().unwrap();

        self.players.get_mut(&self.active_seat).unwrap().active_tile = Some(tile);
    }

    pub fn reset_round(&mut self, settings: &Settings) {
        self.deck = Deck::new(&settings.game);

        let mut rng = rand::rng();
        let dice_roll: usize = rng.random_range(1..=6) + rng.random_range(1..=6);
        self.deck.init_index(dice_roll);

        // TODO: 2x2 dealing hands
        for direction in Direction::iter() {
            let wind = Wind::from_repr(direction.clone() as usize).unwrap();
            let mut player = Player::new(direction.clone(), wind);
            player.add_tiles(self.deck.draw_tiles(settings.game.hand_size));

            self.players.insert(direction, player);
        }

        self.round += 1;
    }

    pub fn update(&mut self, _settings: &Settings, message: GameMessage) {
        match message {
            GameMessage::SortHand => {
                self.players.get_mut(&self.active_seat).unwrap().hand.sort();
            }
            GameMessage::TileClick(id, direction) => {
                println!("{}, {:?}", id, direction);
                if direction != self.active_seat {
                    return;
                }

                self.players.get_mut(&direction).unwrap().play_tile(id);
                self.active_seat = self.active_seat.next();
                self.draw_tile();
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
        elements.extend(
            self.players
                .iter()
                .flat_map(|(_, player)| player.render(settings)),
        );
        elements.extend(render_buttons(settings));

        Stack::from_vec(elements)
            .width(settings.video.window_size.width)
            .height(settings.video.window_size.height)
            .into()
    }
}
