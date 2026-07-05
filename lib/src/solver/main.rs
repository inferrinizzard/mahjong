use crate::{
    solver::shanten::{Shanten, solve_shanten},
    tile::TileCounts,
};

pub struct SolveResult {
    shanten: Shanten,
    is_tenpai: bool,
    // ukeire: Tile
}

pub fn solve(
    tiles: &TileCounts,
    _unavailable_tiles: Option<&TileCounts>,
    num_wilds: usize,
) -> SolveResult {
    let shanten = solve_shanten(tiles, num_wilds);

    SolveResult {
        shanten,
        is_tenpai: shanten == 0,
    }
}

// pub fn is_tenpai() -> bool {}
