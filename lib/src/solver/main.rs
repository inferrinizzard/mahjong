use crate::{
    maps::TileFrequency,
    solver::shanten::{solve_shanten, Shanten},
    structs::Hand,
};

pub struct SolveResult {
    shanten: Shanten,
    is_tenpai: bool,
    // ukeire: Tile
}

pub fn solve(hand: &Hand, unavailable_tiles: Option<&TileFrequency>) -> SolveResult {
    let shanten = solve_shanten(&hand.tile_frequency, hand.num_wilds);

    SolveResult {
        shanten,
        is_tenpai: shanten == 0,
    }
}

// pub fn is_tenpai() -> bool {}
