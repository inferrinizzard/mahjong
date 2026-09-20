use clap::Parser as ClapParser;
use mahjong_lib::{
    notation::Parser as MahjongParser, solver::solve_shanten, tile::TILE_UNICODE_LIST,
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

    let is_valid_hand = MahjongParser::is_valid(&hand);
    if !is_valid_hand {
        println!("Invalid hand: {}", hand);
        println!("Ensure it is in mpsz algebraic notation, ex: 123m456p789s55511z");
        return;
    }

    let tile_counts = MahjongParser::parse_to_counts(hand).unwrap();

    println!(
        "Hand: {}",
        tile_counts
            .iter()
            .enumerate()
            .flat_map(|(i, count)| vec![TILE_UNICODE_LIST[i]; *count])
            .collect::<Vec<&str>>()
            .join("")
    );

    let shanten = solve_shanten(&tile_counts);

    println!("Shanten: {}", shanten);
    if shanten == -1 {
        println!("Winning Hand");
    }
    if shanten == 0 {
        println!("Tenpai: {}", "ukeire here");
    }
}
