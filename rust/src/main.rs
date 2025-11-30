use crate::game::{create_deck, game::create_game, DeckFlags};

#[macro_use]
extern crate lazy_static;

mod consts;
mod game;
mod maps;
mod structs;
mod traits;
mod types;

fn main() {
    println!("Hello, world!");
    let game = create_game();
}
