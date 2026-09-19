use crate::tile::TileData;

pub enum MeldType {
    PONG,
    CHI,
    GANG,
    ANKAN,
}

pub struct Meld {
    pub meld_type: MeldType,
    pub tiles: Vec<TileData>,
}

impl From<Vec<TileData>> for Meld {
    fn from(value: Vec<TileData>) -> Self {
        if value.len() > 3 {
            return Meld {
                meld_type: MeldType::GANG,
                tiles: value.into_iter().take(4).collect(),
            };
        }
        if value[0] == value[1] {
            return Meld {
                meld_type: MeldType::PONG,
                tiles: value.into_iter().take(3).collect(),
            };
        }
        return Meld {
            meld_type: MeldType::CHI,
            tiles: value.into_iter().take(3).collect(),
        };
    }
}
