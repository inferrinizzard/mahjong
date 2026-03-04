pub struct Branch {
    num_tiles: usize,

    quads: Vec<(u8, u8, u8, u8)>,
    melds: Vec<(u8, u8, u8)>,
    pairs: Vec<(u8, u8)>,
    tatsu: Vec<(u8, u8)>,
    singles: Vec<u8>,

    score: usize,
}

impl Branch {
    pub fn new() -> Branch {
        Branch {
            num_tiles: 0,
            quads: vec![],
            melds: vec![],
            pairs: vec![],
            tatsu: vec![],
            singles: vec![],
            score: 0,
        }
    }

    pub fn add_item(&mut self, nums: Vec<u8>) {
        let num_count = nums.len();

        self.num_tiles = self.num_tiles - num_count;
        match num_count {
            4 => self.quads.push((nums[0], nums[1], nums[2], nums[3])),
            3 => self.melds.push((nums[0], nums[1], nums[2])),
            2 => {
                if nums[0] == nums[1] {
                    self.pairs.push((nums[0], nums[1]))
                } else {
                    self.tatsu.push((nums[0], nums[1]))
                }
            }
            1 => self.singles.push(nums[0]),
            _ => {}
        }
    }

    pub fn calculate_score(&mut self) -> usize {
        let mut sum = 0;
        sum += self.quads.len() * 5;
        sum += self.melds.len() * 5;
        sum += self.pairs.len() * 3;
        sum += self.tatsu.len() * 1;
        sum -= self.singles.len() * 1;

        self.score = sum;
        sum
    }
}

// impl From<String> for Branch {
//     fn from(value: String) -> Self {}
// }

// impl ToString for Branch {}
