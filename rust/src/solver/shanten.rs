use std::cmp;

use crate::{consts::TileName, maps::TileFrequency};

lazy_static! {
    static ref THIRTEEN_ORPHANS: Vec<TileName> = vec![
        TileName::MAN_1,
        TileName::MAN_9,
        TileName::BAMBOO_1,
        TileName::BAMBOO_9,
        TileName::TONG_1,
        TileName::TONG_9,
        TileName::WIND_EAST,
        TileName::WIND_SOUTH,
        TileName::WIND_WEST,
        TileName::WIND_NORTH,
        TileName::DRAGON_GREEN,
        TileName::DRAGON_RED,
        TileName::DRAGON_WHITE,
    ];
}

pub type Shanten = i8;

pub fn solve_shanten(tile_frequency: &TileFrequency, num_wilds: u8) -> Shanten {
    let standard_shanten = solve_standard_shanten(tile_frequency);
    let seven_pairs_shanten = solve_seven_pairs_shanten(tile_frequency);
    let thirteen_orphans_shanten = solve_thirteen_orphans_shanten(tile_frequency);

    cmp::max(
        cmp::min(
            cmp::min(standard_shanten, seven_pairs_shanten),
            thirteen_orphans_shanten,
        ) - num_wilds as i8, // don't subtract wild if thirteen orphan
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

    let mut num_different_terminals = 0;
    let mut num_terminal_pairs = 0;
    THIRTEEN_ORPHANS.iter().for_each(|orphan| {
        let count = tile_frequency.map[orphan];
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
