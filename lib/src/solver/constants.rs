use crate::Tile;

lazy_static! {
    pub static ref THIRTEEN_ORPHANS: Vec<usize> = vec![
        Tile::MAN_1.index as usize,
        Tile::MAN_9.index as usize,
        Tile::BAMBOO_1.index as usize,
        Tile::BAMBOO_9.index as usize,
        Tile::TONG_1.index as usize,
        Tile::TONG_9.index as usize,
        Tile::EAST_WIND.index as usize,
        Tile::SOUTH_WIND.index as usize,
        Tile::WEST_WIND.index as usize,
        Tile::NORTH_WIND.index as usize,
        Tile::GREEN_DRAGON.index as usize,
        Tile::RED_DRAGON.index as usize,
        Tile::WHITE_DRAGON.index as usize,
    ];
}

pub const MAX_THIRTEEN_ORPHANS_SHANTEN: i8 = 13;
pub const MAX_SEVEN_PAIRS_SHANTEN: i8 = 6;
pub const MAX_STANDARD_SHANTEN: i8 = 8;
