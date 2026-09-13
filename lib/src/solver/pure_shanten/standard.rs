use crate::{
    solver::{constants::MAX_STANDARD_SHANTEN, main::Shanten},
    tile::TileCounts,
};

pub fn solve_standard_shanten(tile_counts: &TileCounts) -> Shanten {
    let mut shanten = MAX_STANDARD_SHANTEN;

    //  8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))

    shanten
}
