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

pub struct SuitAnalyzer {
    // cache: HashMap<String, Branch>,
    queue: Vec<Branch>,
    leaves: Vec<Branch>,
}

impl SuitAnalyzer {
    pub fn find_decompositions(hand: &SuitTileCounts) -> Vec<Branch> {
        let mut suit_analyzer = SuitAnalyzer::new(hand);
        suit_analyzer.run();
        suit_analyzer.leaves
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
                let key = current_branch.to_string();
                // self.cache.insert(key, current_branch.clone());
                self.leaves.push(current_branch.clone());
                continue;
            }

            let count = current_branch.head_value();

            if count >= 4 {
                // AAA, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::Straight],
                    );
                }
                // AAA, AB_
                else if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::AdjTaatsu],
                    );
                }
                // AAA, A_C
                else if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Triple, BranchAction::SkipTaatsu],
                    );
                }
                // AAAA
                else {
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
                else if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Pair, BranchAction::AdjTaatsu],
                    );
                }
                // AA, A_C
                else if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Pair, BranchAction::SkipTaatsu],
                    );
                }
                // AAA
                else {
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
                else if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Single, BranchAction::AdjTaatsu],
                    );
                }
                // A, A_C
                else if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(
                        &current_branch,
                        vec![BranchAction::Single, BranchAction::SkipTaatsu],
                    );
                }
                // AA
                else {
                    self.split_branch(&current_branch, vec![BranchAction::Pair]);
                }
            } else if count >= 1 {
                // ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::Straight]);
                }
                // AB_
                else if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::AdjTaatsu]);
                }
                // A_C
                else if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {
                    self.split_branch(&current_branch, vec![BranchAction::SkipTaatsu]);
                }
                // A
                else {
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
