use std::collections::HashMap;

use super::branch::Branch;

pub struct BranchSolver {
    branch_map: HashMap<String, Branch>,

    branches: Vec<Branch>,
    leaves: Vec<Branch>,

    cur_branch: Option<Branch>,
}

impl BranchSolver {
    pub fn new() -> BranchSolver {
        BranchSolver {
            branch_map: HashMap::new(),
            branches: vec![],
            leaves: vec![],
            cur_branch: None,
        }
    }
}
