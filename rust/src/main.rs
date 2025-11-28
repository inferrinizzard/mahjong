#[macro_use]
extern crate lazy_static;

mod consts;
mod maps;
mod structs;
mod traits;
mod types;

use structs::Tile;

use crate::consts::{tile::TileNumber, TileData};

fn main() {
    println!("Hello, world!");
    println!("{}", Tile::new(TileData::MAN(TileNumber::ONE)).to_string());
}
