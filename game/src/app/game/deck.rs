use crate::app::{
    game::{GameTile, init_deck::init_deck},
    settings::GameSettings,
};

#[derive(Default)]
pub struct Deck {
    len: usize,
    pub banks: [Vec<Option<GameTile>>; 4],
    pub bank_size: usize,

    pub count: usize,

    pub head_index: usize,
    pub tail_index: usize,
}

impl Deck {
    pub fn new(game_settings: &GameSettings) -> Self {
        let raw_deck = init_deck(game_settings);

        let bank_size = raw_deck.len() / 4;

        let banks = [
            raw_deck[..bank_size]
                .iter()
                .map(|t| Some(t.clone()))
                .collect(),
            raw_deck[bank_size..bank_size * 2]
                .iter()
                .map(|t| Some(t.clone()))
                .collect(),
            raw_deck[bank_size * 2..bank_size * 3]
                .iter()
                .map(|t| Some(t.clone()))
                .collect(),
            raw_deck[bank_size * 3..]
                .iter()
                .map(|t| Some(t.clone()))
                .collect(),
        ];

        Self {
            len: raw_deck.len(),
            banks,
            bank_size,
            count: raw_deck.len(),

            head_index: 0,
            tail_index: 0,
        }
    }

    pub fn init_index(&mut self, dice_roll: usize) {
        let starting_bank = (dice_roll + 1) % self.banks.len();
        self.head_index = starting_bank * self.bank_size + dice_roll * 2;
        // TODO: offset by 14/15 if using dead wall
        self.tail_index = (self.head_index + self.len - 1) % self.len;
    }

    fn get_tile(&mut self, index: usize) -> GameTile {
        let active_bank = (index / self.bank_size) % self.banks.len();
        let bank_index = (index - (self.bank_size * active_bank)) % self.len;

        let tile = self.banks[active_bank][bank_index].clone();
        self.banks[active_bank][bank_index] = None;
        self.count -= 1;

        tile.unwrap()
    }

    pub fn draw_tiles(&mut self, num_tiles: usize) -> Vec<GameTile> {
        let mut tiles = vec![];
        for _ in 0..num_tiles {
            tiles.push(self.get_tile(self.head_index));
            self.head_index = (self.head_index + 1) % self.len;
        }

        tiles
    }

    pub fn draw_from_rear(&mut self) -> GameTile {
        let tile = self.get_tile(self.tail_index);
        self.tail_index = (self.tail_index + self.len - 1) % self.len;

        tile
    }
}
