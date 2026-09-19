use std::cmp;

use crate::{
    solver::{
        constants::{MAX_THIRTEEN_ORPHANS_SHANTEN, THIRTEEN_ORPHANS},
        main::Shanten,
    },
    tile::TileCounts,
};

/// Find shanten for thirteen orphans, subtract number of existing orphans from max and -1 if pair exists
pub fn solve_thirteen_orphans_shanten(tile_counts: &TileCounts) -> Shanten {
    let mut num_different_terminals = 0;
    let mut has_terminal_pair = false;
    THIRTEEN_ORPHANS.iter().for_each(|orphan| {
        let count = tile_counts[*orphan];
        if count > 0 {
            num_different_terminals += 1;
        }
        if count >= 2 {
            has_terminal_pair = true;
        }
    });

    let mut shanten = cmp::min(MAX_THIRTEEN_ORPHANS_SHANTEN, 13 - num_different_terminals);
    if has_terminal_pair {
        shanten -= 1
    }

    shanten
}
