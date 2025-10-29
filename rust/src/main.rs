#[macro_use]
extern crate lazy_static;

mod consts;
mod maps;
mod structs;
mod traits;
mod types;

use consts::Suit;
use structs::Tile;

fn main() {
    println!("Hello, world!");
    Tile::new(Suit::MAN, 1);
}
