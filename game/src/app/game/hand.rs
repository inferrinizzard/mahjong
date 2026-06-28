use mahjong_lib::tile::{Meld, TileData};

#[derive(Default)]
pub struct GameHand {
    pub closed: Vec<TileData>,
    pub open: Vec<Meld>,
}

impl GameHand {
    pub fn add_tiles() {}
}

impl From<Vec<TileData>> for GameHand {
    fn from(value: Vec<TileData>) -> Self {
        Self {
            closed: value,
            open: vec![],
        }
    }
}
