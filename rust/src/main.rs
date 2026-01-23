use crate::game::{game::create_game, DeckOptions, GameOptions};

#[macro_use]
extern crate lazy_static;

mod consts;
mod game;
mod maps;
mod notation;
mod structs;

fn main() {
    println!("Hello, world!");
    let mut game_options = GameOptions {
        deck: DeckOptions {
            has_akadora: true,
            has_flowers: true,
            has_seasons: true,
            wild: None,
        },
    };

    let game = create_game(&game_options);
}
