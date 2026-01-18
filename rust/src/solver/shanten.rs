use std::cmp;

use crate::structs::Hand;

pub type Shanten = u8;

pub fn solve_shanten(hand: &Hand) -> Shanten {
    let standard_shanten = solve_standard_shanten(hand);
    let seven_pairs_shanten = solve_seven_pairs_shanten(hand);
    let thirteen_orphans_shanten = solve_thirteen_orphans_shanten(hand);

    cmp::min(
        cmp::min(standard_shanten, seven_pairs_shanten),
        thirteen_orphans_shanten,
    )
}

fn solve_standard_shanten(hand: &Hand) -> Shanten {
    let max_standard_shanten = 8;
    let mut shanten = max_standard_shanten;

    //  8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))

    shanten
}

fn solve_seven_pairs_shanten(hand: &Hand) -> Shanten {
    let max_seven_pairs_shanten = 6;
    let mut shanten = max_seven_pairs_shanten;

    //  6 - pairs

    shanten
}

fn solve_thirteen_orphans_shanten(hand: &Hand) -> Shanten {
    let max_thirteen_orphans_shanten = 13;
    let mut shanten = max_thirteen_orphans_shanten;

    // 13 - diffTerminals - min(terminalPairs, 1)

    shanten
}
