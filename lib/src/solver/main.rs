use crate::{solver::shanten::solve_shanten, tile::TileCounts};

pub type Shanten = i8;

pub struct SolveResult {
    pub shanten: Shanten,
    pub is_tenpai: bool,
    // ukeire: Tile
}

pub fn solve(tiles: &TileCounts, _unavailable_tiles: Option<&TileCounts>) -> SolveResult {
    let shanten = solve_shanten(tiles);

    SolveResult {
        shanten,
        is_tenpai: shanten == 0,
    }
}

// pub fn is_tenpai() -> bool {}
