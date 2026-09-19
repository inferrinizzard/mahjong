use std::cmp;

use crate::{
    solver::{constants::MAX_STANDARD_SHANTEN, main::Shanten},
    tile::TileCounts,
};

use super::suit_analyzer::SuitAnalyzer;

pub fn solve_standard_shanten(tile_counts: &TileCounts) -> Shanten {
    let mut shanten = MAX_STANDARD_SHANTEN;

    let man = &tile_counts[0..9].try_into().unwrap();
    let man_decompositions = SuitAnalyzer::find_decompositions(man);
    let tong = &tile_counts[18..27].try_into().unwrap();
    let tong_decompositions = SuitAnalyzer::find_decompositions(tong);
    let bamboo = &tile_counts[9..18].try_into().unwrap();
    let bamboo_decompositions = SuitAnalyzer::find_decompositions(bamboo);
    let honors = &tile_counts[27..];
    let honor_decomposition = SuitAnalyzer::find_static_groupings(honors);

    for m in &man_decompositions {
        for t in &tong_decompositions {
            for b in &bamboo_decompositions {
                // 8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))
                let num_melds = (m.melds.len()
                    + t.melds.len()
                    + b.melds.len()
                    + honor_decomposition.melds.len()) as i8;
                let num_pairs = (m.pairs.len()
                    + t.pairs.len()
                    + b.pairs.len()
                    + honor_decomposition.pairs.len()) as i8;
                let num_taatsu = (m.taatsu.len() + t.taatsu.len() + b.taatsu.len()) as i8;

                let current_shanten = 8
                    - (2 * num_melds)
                    - cmp::min(num_pairs + num_taatsu, 4 - num_melds)
                    - cmp::min(1, cmp::max(0, num_pairs + num_taatsu + num_melds - 4));

                shanten = cmp::min(shanten, current_shanten)
            }
        }
    }

    // use lookup tables
    // split by total tile count,
    // 3-7 is most probable, 8-14, 15-17,

    // prune, by score ?
    // find all decompositions for each suit
    // compile into hands and calculate shanten / heuristic + order
    // calculate joker usage at hand level
    shanten
}
