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
    let mut man_decompositions = SuitAnalyzer::find_decompositions(man);
    let tong = &tile_counts[18..27].try_into().unwrap();
    let mut tong_decompositions = SuitAnalyzer::find_decompositions(tong);
    let bamboo = &tile_counts[9..18].try_into().unwrap();
    let mut bamboo_decompositions = SuitAnalyzer::find_decompositions(bamboo);
    let honors = &tile_counts[27..];
    let mut honor_decomposition = SuitAnalyzer::find_static_groupings(honors);

    let mut combinations = vec![];
    for mut m in &mut man_decompositions {
        for mut t in &mut tong_decompositions {
            for mut b in &mut bamboo_decompositions {
                combinations.push(
                    honor_decomposition
                        .merge(&mut m)
                        .merge(&mut t)
                        .merge(&mut b),
                );
            }
        }
    }

    combinations.sort_by(|a, b| {
        let a_shanten = a.calculate_shanten();
        let b_shanten = b.calculate_shanten();

        if b_shanten < a_shanten {
            return std::cmp::Ordering::Greater;
        }
        if a_shanten > b_shanten {
            return std::cmp::Ordering::Less;
        }

        std::cmp::Ordering::Equal
    });

    // use lookup tables
    // split by total tile count,
    // 3-7 is most probable, 8-14, 15-17,

    // prune, by score ?
    // find all decompositions for each suit
    // compile into hands and calculate shanten / heuristic + order
    // calculate joker usage at hand level

    shanten
}
