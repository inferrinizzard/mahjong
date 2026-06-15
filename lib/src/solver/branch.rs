use crate::notation::types::TileHandSuitSubString;

#[derive(Clone)]
pub struct Branch {
    hand: Vec<usize>,
    num_tiles: usize,

    pub quads: Vec<usize>,
    pub melds: Vec<usize>,
    pub pairs: Vec<usize>,
    pub tatsu: Vec<usize>,
    pub singles: Vec<usize>,

    pub score: usize,
}

impl Branch {
    pub fn new(hand: Vec<usize>) -> Branch {
        let num_tiles: usize = hand.iter().sum();

        Branch {
            hand,
            num_tiles,
            quads: vec![],
            melds: vec![],
            pairs: vec![],
            tatsu: vec![],
            singles: vec![],
            score: 0,
        }
    }

    pub fn to_hand_suit_sub_string(&self) -> TileHandSuitSubString {
        self.hand
            .iter()
            .map(|count| count.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    // pub fn add_item(&mut self, nums: Vec<usize>) {
    //     let num_count = nums.len();

    //     self.num_tiles = self.num_tiles - num_count;
    //     match num_count {
    //         4 => self.quads.push((nums[0], nums[1], nums[2], nums[3])),
    //         3 => self.melds.push((nums[0], nums[1], nums[2])),
    //         2 => {
    //             if nums[0] == nums[1] {
    //                 self.pairs.push((nums[0], nums[1]))
    //             } else {
    //                 self.tatsu.push((nums[0], nums[1]))
    //             }
    //         }
    //         1 => self.singles.push(nums[0]),
    //         _ => {}
    //     }
    // }

    pub fn calculate_scores(&mut self) -> usize {
        let mut sum = 0;
        sum += self.quads.len() * 5;
        sum += self.melds.len() * 5;
        sum += self.pairs.len() * 3;
        sum += self.tatsu.len() * 1;
        sum -= self.singles.len() * 1;

        self.score = sum;
        sum

        //^above score maximises fastest shanten
        // also calculate tile efficiency score for ukeire, based on wait types
        // maximise based on score after
        //
    }

    pub fn is_empty(&self) -> bool {
        self.num_tiles == 0
    }
}

impl From<TileHandSuitSubString> for Branch {
    fn from(value: TileHandSuitSubString) -> Self {
        let tiles = value
            .split("")
            .map(|s| str::parse::<usize>(s).unwrap())
            .collect();
        Branch::new(tiles)
    }
}

// impl ToString for Branch {
//     fn to_string(&self) -> String {
//         ""
//     }
// }
