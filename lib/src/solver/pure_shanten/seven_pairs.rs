use crate::{
    solver::{constants::MAX_SEVEN_PAIRS_SHANTEN, main::Shanten},
    tile::TileCounts,
};

/// Find shanten for seven pairs winning strategy, subtract num pairs from max count
pub fn solve_seven_pairs_shanten(tile_counts: &TileCounts) -> Shanten {
    let num_pairs: usize = tile_counts.iter().map(|entry| entry / 2).sum();

    let shanten = MAX_SEVEN_PAIRS_SHANTEN - (num_pairs as i8);
    shanten
}
