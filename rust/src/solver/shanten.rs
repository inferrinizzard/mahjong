use std::cmp;

use crate::maps::TileFrequency;

pub type Shanten = i8;

pub fn solve_shanten(tile_frequency: &TileFrequency, num_wilds: u8) -> Shanten {
    let standard_shanten = solve_standard_shanten(tile_frequency);
    let seven_pairs_shanten = solve_seven_pairs_shanten(tile_frequency);
    let thirteen_orphans_shanten = solve_thirteen_orphans_shanten(tile_frequency);

    cmp::max(
        cmp::min(
            cmp::min(standard_shanten, seven_pairs_shanten),
            thirteen_orphans_shanten,
        ) - num_wilds as i8,
        -1,
    )
}

fn solve_standard_shanten(tile_frequency: &TileFrequency) -> Shanten {
    let max_standard_shanten = 8;
    let mut shanten = max_standard_shanten;

    //  8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))

    shanten
}

fn solve_seven_pairs_shanten(tile_frequency: &TileFrequency) -> Shanten {
    let max_seven_pairs_shanten = 6;
    let mut shanten = max_seven_pairs_shanten;

    let num_pairs: u8 = tile_frequency
        .map
        .iter()
        // .filter(|entry| *entry.1 >= 2)
        .map(|entry| *entry.1 / 2)
        .sum();

    shanten = cmp::min(shanten, max_seven_pairs_shanten - num_pairs);
    shanten as i8
}

fn solve_thirteen_orphans_shanten(tile_frequency: &TileFrequency) -> Shanten {
    let max_thirteen_orphans_shanten = 13;
    let mut shanten = max_thirteen_orphans_shanten;

    // 13 - diffTerminals - min(terminalPairs, 1)

    shanten
}
