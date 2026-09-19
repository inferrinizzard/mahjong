use std::cmp;

use crate::solver::pure_shanten::suit_analyzer::{BranchAction, SuitTileCounts};

///
#[derive(Debug, Clone)]
pub struct Branch {
    hand: SuitTileCounts,

    pub melds: Vec<[usize; 3]>,
    pub pairs: Vec<[usize; 2]>,
    pub taatsu: Vec<[usize; 2]>,
    pub singles: Vec<usize>,
}

impl Branch {
    pub fn new(hand: SuitTileCounts) -> Branch {
        Branch {
            hand,

            melds: vec![],
            pairs: vec![],
            taatsu: vec![],
            singles: vec![],
        }
    }

    fn get_index(&self) -> usize {
        let next_index = self.hand.iter().position(|x| *x > 0);
        match next_index {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.hand.iter().sum::<usize>() == 0
    }

    pub fn get_head_value(&self) -> usize {
        self.hand[self.get_index()]
    }

    pub fn matches_kernel(&self, kernel: [usize; 3]) -> bool {
        let start_index = self.get_index();
        kernel
            .iter()
            .enumerate()
            .all(|(i, x)| self.hand[start_index + i] >= *x)
    }

    fn remove_tiles(&mut self, tiles: &[usize]) {
        for tile in tiles {
            if self.hand[*tile] == 0 {
                panic!("Invalid tile");
            }

            self.hand[*tile] -= 1;
        }
    }

    pub fn add_triple_at_index(&mut self) {
        let triple = [self.get_index(); 3];
        self.add_meld(triple);
    }

    pub fn add_straight_at_index(&mut self) {
        let index = self.get_index();
        let straight = [index, index + 1, index + 2];
        self.add_meld(straight);
    }

    fn add_meld(&mut self, meld: [usize; 3]) {
        self.remove_tiles(&meld);
        self.melds.push(meld);
    }

    pub fn add_pair_at_index(&mut self) {
        let pair = [self.get_index(); 2];
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
            BranchAction::AdjTaatsu => [self.get_index(), self.get_index() + 1],
            BranchAction::SkipTaatsu => [self.get_index(), self.get_index() + 2],
            _ => [0, 0],
        };

        self.remove_tiles(&taatsu);
        self.pairs.push(taatsu);
    }

    pub fn add_single_at_index(&mut self) {
        let tile = self.get_index();
        self.remove_tiles(&[tile]);
        self.singles.push(tile);
    }

    pub fn calculate_shanten(&self) -> i8 {
        // 8 - (2 * groups) - min(pairs + taatsu, 4 - groups) - min(1, max(0, pairs + taatsu + groups - 4))
        let num_melds = self.melds.len() as i8;
        let num_pairs = self.pairs.len() as i8;
        let num_taatsu = self.taatsu.len() as i8;

        let shanten = 8
            - (2 * num_melds)
            - cmp::min(num_pairs, num_taatsu)
            - cmp::min(1, cmp::max(0, num_pairs + num_taatsu + num_melds - 4));

        shanten as i8
    }
}

impl ToString for Branch {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.hand.iter().for_each(|x| s += &x.to_string());
        s
    }
}
