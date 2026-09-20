mod branch;
mod seven_pairs;
mod standard;
mod suit_analyzer;
mod thirteen_orphans;

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

pub fn solve_shanten(tile_counts: &TileCounts) -> i8 {
    let total_num_tiles = tile_counts.iter().sum();

    let mut seven_pairs_shanten = MAX_SEVEN_PAIRS_SHANTEN;
    let mut thirteen_orphans_shanten = MAX_THIRTEEN_ORPHANS_SHANTEN;

    if matches!(total_num_tiles, 13 | 14) {
        seven_pairs_shanten = solve_seven_pairs_shanten(tile_counts);
        thirteen_orphans_shanten = solve_thirteen_orphans_shanten(tile_counts);
    }

    let standard_shanten = solve_standard_shanten(tile_counts);

    log::debug!(
        "standard: {}, seven_pairs: {}, thirteen_orphans: {}",
        standard_shanten,
        seven_pairs_shanten,
        thirteen_orphans_shanten
    );

    cmp::min(
        cmp::min(seven_pairs_shanten, thirteen_orphans_shanten),
        standard_shanten,
    )
}
