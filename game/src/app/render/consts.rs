use strum::EnumCount;
use strum_macros::{EnumCount, EnumIter, FromRepr};

use mahjong_lib::traits::Next;

pub static TILE_BACK_PATH: &'static str = "assets/tiles/oblique/misc/back.svg";
pub static TILE_BACK_X_PATH: &'static str = "assets/tiles/oblique/misc/back_x.svg";

pub static TILE_ASPECT_RATIO: f32 = 78. / 64.;
pub static TILE_FACE_RATIO: f32 = 50. / 64.;
pub static TILE_EDGE_RATIO: f32 = 1. - TILE_FACE_RATIO;

pub fn get_total_tile_length(num_tiles: usize, size: u32) -> f32 {
    (TILE_FACE_RATIO * num_tiles as f32 + TILE_EDGE_RATIO) * size as f32
}

#[derive(Debug, Default, Clone, Hash, EnumIter, PartialEq, Eq, EnumCount, FromRepr)]
pub enum Direction {
    #[default]
    DOWN,
    RIGHT,
    UP,
    LEFT,
}

impl Next for Direction {
    fn next(&self) -> Self {
        Direction::from_repr((self.clone() as usize + 1) % Direction::COUNT).unwrap()
    }
}
