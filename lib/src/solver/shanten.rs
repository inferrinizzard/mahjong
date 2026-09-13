use std::cmp;

use crate::{
    solver::{constants::THIRTEEN_ORPHANS, main::Shanten},
    tile::TileCounts,
};

pub fn solve_shanten(tile_counts: &TileCounts) -> Shanten {
    let standard_shanten = solve_standard_shanten(tile_counts);
    let seven_pairs_shanten = solve_seven_pairs_shanten(tile_counts);
    let thirteen_orphans_shanten = solve_thirteen_orphans_shanten(tile_counts);

    cmp::max(
        cmp::min(
            cmp::min(standard_shanten, seven_pairs_shanten),
            thirteen_orphans_shanten,
        ),
        -1,
    )
}

fn solve_standard_shanten(tile_counts: &TileCounts) -> Shanten {
    let max_standard_shanten = 8;
    let mut shanten = max_standard_shanten;

    //  8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))

    shanten
}

fn solve_seven_pairs_shanten(tile_counts: &TileCounts) -> Shanten {
    let max_seven_pairs_shanten = 6;

    let num_pairs: usize = tile_counts
        .iter()
        // .filter(|entry| *entry.1 >= 2)
        .map(|entry| entry / 2)
        .sum();

    let shanten = max_seven_pairs_shanten - num_pairs;
    shanten as i8
}

fn solve_thirteen_orphans_shanten(tile_counts: &TileCounts) -> Shanten {
    let max_thirteen_orphans_shanten = 13;

    let mut num_different_terminals = 0;
    let mut num_terminal_pairs = 0;
    THIRTEEN_ORPHANS.iter().for_each(|orphan| {
        let count = tile_counts[*orphan];
        if count > 0 {
            num_different_terminals += 1;
        }
        if count >= 2 {
            num_terminal_pairs += 1;
        }
    });

    let shanten = cmp::min(
        max_thirteen_orphans_shanten,
        13 - num_different_terminals - cmp::min(num_terminal_pairs, 1),
    );
    shanten
}
