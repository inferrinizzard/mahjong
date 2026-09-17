use std::cmp::{self, max};

use crate::solver::pure_shanten::suit_analyzer::{BranchAction, SuitTileCounts};

///
#[derive(Debug, Clone)]
pub struct Branch {
    hand: SuitTileCounts,
    index: usize,
    num_tiles: usize,

    melds: Vec<[usize; 3]>,
    pairs: Vec<[usize; 2]>,
    taatsu: Vec<[usize; 2]>,
    singles: Vec<usize>,
}

impl Branch {
    pub fn new(hand: SuitTileCounts) -> Branch {
        Branch {
            hand,
            index: *hand.iter().find(|x| **x > 0).unwrap(),
            num_tiles: hand.iter().sum(),

            melds: vec![],
            pairs: vec![],
            taatsu: vec![],
            singles: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.num_tiles == 0
    }

    pub fn head_value(&self) -> usize {
        self.hand[self.index]
    }

    pub fn matches_kernel(&self, kernel: [usize; 3]) -> bool {
        kernel
            .iter()
            .enumerate()
            .all(|(x, i)| self.hand[self.index + i] >= x)
    }

    fn remove_tiles(&mut self, tiles: &[usize]) {
        for tile in tiles {
            if self.hand[*tile] == 0 {
                panic!("Invalid tile")
            }

            self.hand[*tile] -= 1;
        }
    }

    pub fn add_triple_at_index(&mut self) {
        let meld = [self.index; 3];
        self.add_meld(meld);
    }

    pub fn add_straight_at_index(&mut self) {
        let meld = [self.index, self.index + 1, self.index + 2];
        self.add_meld(meld);
    }

    fn add_meld(&mut self, meld: [usize; 3]) {
        self.remove_tiles(&meld);
        self.melds.push(meld);
    }

    pub fn add_pair_at_index(&mut self) {
        let pair = [self.index; 2];
        self.remove_tiles(&pair);
        self.pairs.push(pair);
    }

    pub fn add_taatsu_at_index(&mut self, taatsu_type: BranchAction) {
        if !matches!(
            taatsu_type,
            BranchAction::AdjTaatsu | BranchAction::SkipTaatsu
        ) {
            return;
        }

        let taatsu = match taatsu_type {
            BranchAction::AdjTaatsu => [self.index, self.index + 1],
            BranchAction::SkipTaatsu => [self.index, self.index + 2],
            _ => [0, 0],
        };

        self.remove_tiles(&taatsu);
        self.pairs.push(taatsu);
    }

    pub fn add_single_at_index(&mut self) {
        let tile = self.index;
        self.remove_tiles(&[tile]);
        self.singles.push(tile);
    }

    pub fn merge(&mut self, branch: &mut Branch) -> Branch {
        self.melds.append(&mut branch.melds);
        self.pairs.append(&mut branch.pairs);
        self.taatsu.append(&mut branch.taatsu);
        self.singles.append(&mut branch.singles);

        self.clone()
    }

    pub fn calculate_shanten(&self) -> usize {
        // 8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))
        let num_melds = self.melds.len();
        let num_pairs = self.pairs.len();
        let num_taatsu = self.taatsu.len();

        let shanten = 8
            - (2 * num_melds)
            - cmp::min(num_pairs, num_taatsu)
            - cmp::min(1, max(0, num_pairs + num_taatsu + num_melds + 4));

        shanten
    }
}

impl ToString for Branch {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.hand.iter().for_each(|x| s += &x.to_string());
        s
    }
}
