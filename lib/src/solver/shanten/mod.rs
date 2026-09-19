use std::cmp;

use log;

use crate::{
    solver::{
        constants::{MAX_SEVEN_PAIRS_SHANTEN, MAX_THIRTEEN_ORPHANS_SHANTEN},
        shanten::{
            seven_pairs::solve_seven_pairs_shanten, standard::solve_standard_shanten,
            thirteen_orphans::solve_thirteen_orphans_shanten,
        },
    },
    tile::TileCounts,
};

mod branch;
mod seven_pairs;
mod standard;
mod suit_analyzer;
mod thirteen_orphans;

pub fn solve_shanten(tile_counts: &TileCounts) -> i8 {
    let total_num_tiles = tile_counts.iter().sum();

    let seven_pairs_shanten = if matches!(total_num_tiles, 13 | 14) {
        solve_seven_pairs_shanten(tile_counts)
    } else {
        MAX_SEVEN_PAIRS_SHANTEN
    };
    let thirteen_orphans_shanten = if matches!(total_num_tiles, 13 | 14) {
        solve_thirteen_orphans_shanten(tile_counts)
    } else {
        MAX_THIRTEEN_ORPHANS_SHANTEN
    };
    let standard_shanten = solve_standard_shanten(tile_counts);

    log::debug!(
        "standard shanten: {}, seven_pairs shanten: {}, thirteen_orphans shanten: {}",
        standard_shanten,
        seven_pairs_shanten,
        thirteen_orphans_shanten
    );
    cmp::min(
        cmp::min(seven_pairs_shanten, thirteen_orphans_shanten),
        standard_shanten,
    )
}
