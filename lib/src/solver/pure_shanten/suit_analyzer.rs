use std::collections::HashMap;

use crate::solver::{
    constants::{ADJ_TAATSU_KERNEL, SKIP_TAATSU_KERNEL, STRAIGHT_KERNEL},
    pure_shanten::branch::Branch,
};

pub type SuitTileCounts = [usize; 9];

pub struct SuitAnalyzer {
    cache: HashMap<String, Branch>,
    queue: Vec<Branch>,
    leaves: Vec<Branch>,
}

impl SuitAnalyzer {
    pub fn analyze(hand: &SuitTileCounts) {}

    pub fn new(hand: &SuitTileCounts) -> Self {
        Self {
            cache: HashMap::new(),
            queue: vec![Branch::new(hand.clone())],
            leaves: vec![],
        }
    }

    fn parse(&mut self) {
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
                self.cache.insert(key, current_branch.clone());
                self.leaves.push(current_branch.clone());
                continue;
            }

            let count = current_branch.head_value();

            if count >= 4 {
                // AAA, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {}
                // AAA, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {}
                // AAA, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {}
                // AAAA
            } else if count >= 3 {
                // AA, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {}
                // AA, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {}
                // AA, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {}
                // AAA
            } else if count >= 2 {
                // A, ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {}
                // A, AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {}
                // A, A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {}
                // AA
            } else if count >= 1 {
                // ABC
                if current_branch.matches_kernel(STRAIGHT_KERNEL) {}
                // AB_
                if current_branch.matches_kernel(ADJ_TAATSU_KERNEL) {}
                // A_C
                if current_branch.matches_kernel(SKIP_TAATSU_KERNEL) {}
                // A
            }
        }
    }

    fn split_branch(branch: &Branch) {}
}
