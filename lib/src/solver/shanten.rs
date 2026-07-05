use std::cmp;

use crate::{Tile, tile::TileCounts};

lazy_static! {
    static ref THIRTEEN_ORPHANS: Vec<usize> = vec![
        Tile::MAN_1.index as usize,
        Tile::MAN_9.index as usize,
        Tile::BAMBOO_1.index as usize,
        Tile::BAMBOO_9.index as usize,
        Tile::TONG_1.index as usize,
        Tile::TONG_9.index as usize,
        Tile::EAST_WIND.index as usize,
        Tile::SOUTH_WIND.index as usize,
        Tile::WEST_WIND.index as usize,
        Tile::NORTH_WIND.index as usize,
        Tile::GREEN_DRAGON.index as usize,
        Tile::RED_DRAGON.index as usize,
        Tile::WHITE_DRAGON.index as usize,
    ];
}

pub type Shanten = i8;

pub fn solve_shanten(tile_counts: &TileCounts, num_wilds: usize) -> Shanten {
    let standard_shanten = solve_standard_shanten(tile_counts);
    let seven_pairs_shanten = solve_seven_pairs_shanten(tile_counts);
    let thirteen_orphans_shanten = solve_thirteen_orphans_shanten(tile_counts);

    cmp::max(
        cmp::min(
            cmp::min(standard_shanten, seven_pairs_shanten),
            thirteen_orphans_shanten,
        ) - num_wilds as i8, // don't subtract wild if thirteen orphan
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
    let mut shanten = max_seven_pairs_shanten;

    let num_pairs: usize = tile_counts
        .iter()
        // .filter(|entry| *entry.1 >= 2)
        .map(|entry| entry / 2)
        .sum();

    shanten = cmp::min(shanten, max_seven_pairs_shanten - num_pairs);
    shanten as i8
}

fn solve_thirteen_orphans_shanten(tile_counts: &TileCounts) -> Shanten {
    let max_thirteen_orphans_shanten = 13;
    let mut shanten = max_thirteen_orphans_shanten;

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

    shanten = cmp::min(
        shanten,
        13 - num_different_terminals - cmp::min(num_terminal_pairs, 1),
    );
    shanten
}
