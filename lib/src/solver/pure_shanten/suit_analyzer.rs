use std::collections::HashMap;

use crate::solver::{
    constants::{ADJ_TAATSU_KERNEL, SKIP_TAATSU_KERNEL, STRAIGHT_KERNEL},
    pure_shanten::branch::Branch,
};

pub type SuitTileCounts = [usize; 9];

pub enum BranchAction {
    Quad,
    Triple,
    Straight,
    Pair,
    AdjTaatsu,
    SkipTaatsu,
    Single,
}

#[derive(Debug)]
pub struct SuitAnalyzer {
    // cache: HashMap<String, Branch>,
    queue: Vec<Branch>,
    leaves: Vec<Branch>,
}

impl SuitAnalyzer {
    pub fn find_decompositions(hand: &SuitTileCounts) -> Vec<Branch> {
        let mut suit_analyzer = SuitAnalyzer::new(hand);
        suit_analyzer.run();
        println!("{:?}, {:?}", hand, suit_analyzer);
        suit_analyzer.leaves
    }

    pub fn find_static_groupings(hand: &[usize]) -> Branch {
        let mut sized_hand = [0; 9];
        sized_hand[..hand.len()].copy_from_slice(hand);
        let mut branch = Branch::new(sized_hand);
        for tile in hand {
            match tile {
                4 => (),
                3 => branch.add_triple_at_index(),
                2 => branch.add_pair_at_index(),
                1 => branch.add_single_at_index(),
                _ => (),
            }
        }

        branch
    }

    pub fn new(hand: &SuitTileCounts) -> Self {
        Self {
            // cache: HashMap::new(),
            queue: vec![Branch::new(hand.clone())],
            leaves: vec![],
        }
    }

    fn run(&mut self) {
        while !self.queue.is_empty() {
            let current_branch = self.queue.pop().unwrap();

            // check cache
            // if self.cache.contains_key(&current_branch.to_string()) {
            //     continue;
            // }

            // prune by score / heuristic

            // check if all tiles parsed
            if current_branch.is_empty() {
                // let key = current_branch.to_string();
                // self.cache.insert(key, current_branch.clone());
                self.leaves.push(current_branch.clone());
                continue;
            }

            let count = current_branch.get_head_value();

            if count >= 4 {
                // AAA, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::Straight],
                    );
                }
                // AAA, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::AdjTaatsu],
                    );
                }
                // AAA, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::SkipTaatsu],
                    );
                }
                // AAAA
                {
                    self.split_branch(&current_branch, vec![BranchAction::Quad]);
                }
            } else if count >= 3 {
                // AA, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Pair, BranchAction::Straight],
                    );
                }
                // AA, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Pair, BranchAction::AdjTaatsu],
                    );
                }
                // AA, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Pair, BranchAction::SkipTaatsu],
                    );
                }
                // AAA
                {
                    self.split_branch(&current_branch, vec![BranchAction::Triple]);
                }
            } else if count >= 2 {
                // A, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Single, BranchAction::Straight],
                    );
                }
                // A, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Single, BranchAction::AdjTaatsu],
                    );
                }
                // A, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Single, BranchAction::SkipTaatsu],
                    );
                }
                // AA
                {
                    self.split_branch(&current_branch, vec![BranchAction::Pair]);
                }
            } else if count >= 1 {
                // ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::Straight]);
                }
                // AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::AdjTaatsu]);
                }
                // A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::SkipTaatsu]);
                }
                // A
                {
                    self.split_branch(&current_branch, vec![BranchAction::Single]);
                }
            }
        }
    }

    fn split_branch(&mut self, branch: &Branch, actions: Vec<BranchAction>) {
        let mut new_branch = branch.clone();

        for action in actions {
            match action {
                BranchAction::Triple => new_branch.add_triple_at_index(),
                BranchAction::Straight => new_branch.add_straight_at_index(),
                BranchAction::Pair => new_branch.add_pair_at_index(),
                t @ (BranchAction::AdjTaatsu | BranchAction::SkipTaatsu) => {
                    new_branch.add_taatsu_at_index(t)
                }
                BranchAction::Single => new_branch.add_single_at_index(),
                BranchAction::Quad => (),
            }
        }

        self.queue.insert(0, new_branch);
    }
}
