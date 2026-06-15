use indexmap::{indexmap, IndexMap};

use crate::consts::TileName;

lazy_static! {
    pub static ref NEXT_TILE_MAP: IndexMap<TileName, TileName> = {
        indexmap! {
            TileName::MAN_1 => TileName::MAN_2,
            TileName::MAN_2 => TileName::MAN_3,
            TileName::MAN_3 => TileName::MAN_4,
            TileName::MAN_4 => TileName::MAN_5,
            TileName::MAN_5 => TileName::MAN_6,
            TileName::MAN_6 => TileName::MAN_7,
            TileName::MAN_7 => TileName::MAN_8,
            TileName::MAN_8 => TileName::MAN_9,
            TileName::MAN_9 => TileName::MAN_1,
            TileName::TONG_1 => TileName::TONG_2,
            TileName::TONG_2 => TileName::TONG_3,
            TileName::TONG_3 => TileName::TONG_4,
            TileName::TONG_4 => TileName::TONG_5,
            TileName::TONG_5 => TileName::TONG_6,
            TileName::TONG_6 => TileName::TONG_7,
            TileName::TONG_7 => TileName::TONG_8,
            TileName::TONG_8 => TileName::TONG_9,
            TileName::TONG_9 => TileName::TONG_1,
            TileName::BAMBOO_1 => TileName::BAMBOO_2,
            TileName::BAMBOO_2 => TileName::BAMBOO_3,
            TileName::BAMBOO_3 => TileName::BAMBOO_4,
            TileName::BAMBOO_4 => TileName::BAMBOO_5,
            TileName::BAMBOO_5 => TileName::BAMBOO_6,
            TileName::BAMBOO_6 => TileName::BAMBOO_7,
            TileName::BAMBOO_7 => TileName::BAMBOO_8,
            TileName::BAMBOO_8 => TileName::BAMBOO_9,
            TileName::BAMBOO_9 => TileName::BAMBOO_1,
            TileName::WIND_EAST => TileName::WIND_SOUTH,
            TileName::WIND_SOUTH => TileName::WIND_WEST,
            TileName::WIND_WEST => TileName::WIND_NORTH,
            TileName::WIND_NORTH => TileName::WIND_EAST,
            TileName::DRAGON_WHITE => TileName::DRAGON_GREEN,
            TileName::DRAGON_GREEN => TileName::DRAGON_RED,
            TileName::DRAGON_RED => TileName::DRAGON_WHITE,
            TileName::FLOWER_PLUM => TileName::FLOWER_ORCHID,
            TileName::FLOWER_ORCHID => TileName::FLOWER_CHRYSANTHEMUM,
            TileName::FLOWER_CHRYSANTHEMUM => TileName::FLOWER_BAMBOO,
            TileName::FLOWER_BAMBOO => TileName::FLOWER_PLUM,
            TileName::SEASON_SPRING => TileName::SEASON_SUMMER,
            TileName::SEASON_SUMMER => TileName::SEASON_AUTUMN,
            TileName::SEASON_AUTUMN => TileName::SEASON_WINTER,
            TileName::SEASON_WINTER => TileName::SEASON_SPRING,
        }
    };
}
