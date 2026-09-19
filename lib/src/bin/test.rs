use mahjong_lib::{notation::structs::TileString, solver::solve_shanten, tile::TileCounts};

fn main() {
    let input = "11122233344455m";
    let tile_counts = TileCounts::from(TileString::from(input));

    let shanten = solve_shanten(&tile_counts);

    print!("{}, shanten: {}", input, shanten);
}
