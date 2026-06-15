use std::collections::HashMap;

use crate::notation::types::TileHandSuitSubString;

use super::branch::Branch;

pub struct BranchSolver {
    branch_map: HashMap<String, Branch>,

    branches: Vec<Branch>,
    leaves: Vec<Branch>,

    cur_branch: Option<Branch>,
}

impl BranchSolver {
    pub fn new(tiles: TileHandSuitSubString) -> BranchSolver {
        BranchSolver {
            branch_map: HashMap::new(),
            branches: vec![Branch::from(tiles)],
            leaves: vec![],
            cur_branch: None,
        }
    }
}
