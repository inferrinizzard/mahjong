use crate::{
    solver::{
        constants::MAX_STANDARD_SHANTEN, main::Shanten, pure_shanten::suit_analyzer::SuitAnalyzer,
    },
    tile::TileCounts,
};

pub fn solve_standard_shanten(tile_counts: &TileCounts) -> Shanten {
    let mut shanten = MAX_STANDARD_SHANTEN;

    //  8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))

    let man = &tile_counts[0..9].try_into().unwrap();
    SuitAnalyzer::analyze(man);
    let tong = &tile_counts[18..27];
    let bamboo = &tile_counts[9..18];
    let honors = &tile_counts[27..];

    shanten
}
