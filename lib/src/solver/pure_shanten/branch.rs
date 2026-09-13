use crate::solver::pure_shanten::suit_analyzer::SuitTileCounts;

///
#[derive(Debug, Clone)]
pub struct Branch {
    hand: SuitTileCounts,
    start_index: usize,
    num_tiles: usize,
}

impl Branch {
    pub fn new(hand: SuitTileCounts) -> Branch {
        Branch {
            hand,
            start_index: *hand.iter().find(|x| **x > 0).unwrap(),
            num_tiles: hand.iter().sum(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.num_tiles == 0
    }

    pub fn head_value(&self) -> usize {
        self.hand[self.start_index]
    }

    pub fn matches_kernel(&self, kernel: [usize; 3]) -> bool {
        kernel
            .iter()
            .enumerate()
            .all(|(x, i)| self.hand[self.start_index + i] >= x)
    }
}

impl ToString for Branch {
    fn to_string(&self) -> String {
        let mut s = String::new();
        self.hand.iter().for_each(|x| s += &x.to_string());
        s
    }
}
