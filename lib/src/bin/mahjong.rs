use clap::Parser as ClapParser;
use mahjong_lib::{
    notation::{TILE_CODE_REGEX, structs::TileString},
    solver::solve_shanten,
    tile::TileCounts,
};

#[derive(ClapParser, Debug)]
#[command(version, about)]
struct Args {
    /// // shanten or ukeire
    // command: String,

    /// Tile hand
    hand: String,
}

pub fn main() {
    let args = Args::parse();
    let hand = args.hand;

    let is_valid_hand = TILE_CODE_REGEX.is_match(&hand);
    if !is_valid_hand {
        println!("Invalid hand: {}", hand);
        println!("Ensure it is in mpsz algebraic notation, ex: 123m456p789s55511z");
        return;
    }

    let tile_string = TileString::from(hand);
    let shanten = solve_shanten(&TileCounts::from(tile_string));

    println!("Shanten: {}", shanten);
    if shanten == -1 {
        println!("Winning Hand");
    }
    if shanten == 0 {
        println!("Tenpai: {}", "ukeire here");
    }
}
