#[macro_use]
extern crate lazy_static;

mod consts;
mod init;
mod maps;
mod structs;
mod traits;
mod types;

use crate::init::{create_deck, deck::DeckFlags};

fn main() {
    println!("Hello, world!");
    let deck = create_deck(DeckFlags {
        has_akadora: true,
        has_flowers: true,
        has_seasons: true,
    });

    println!("{}", deck[0])
}
