mod consts;
mod structs;

use consts::Suit;
use structs::Tile;

fn main() {
    println!("Hello, world!");
    Tile::new(Suit::MAN, 1);
}
