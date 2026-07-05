use mahjong_lib::TileData;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, Ord)]
pub struct GameTile {
    pub data: TileData,
    pub id: String,

    pub is_akadora: bool,
}

impl From<TileData> for GameTile {
    fn from(value: TileData) -> Self {
        Self {
            data: value,
            id: Uuid::new_v4().to_string(),

            is_akadora: false,
        }
    }
}

impl PartialEq for GameTile {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl PartialOrd for GameTile {
    fn gt(&self, other: &Self) -> bool {
        self.data > other.data
    }

    fn ge(&self, other: &Self) -> bool {
        self.data >= other.data
    }

    fn lt(&self, other: &Self) -> bool {
        self.data < other.data
    }

    fn le(&self, other: &Self) -> bool {
        self.data <= other.data
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            Some(std::cmp::Ordering::Equal)
        } else if self > other {
            Some(std::cmp::Ordering::Greater)
        } else if self < other {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}
