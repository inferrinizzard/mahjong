use std::collections::HashMap;

pub trait TileCode {
    fn to_tile_code(&self) -> String;
}

lazy_static! {
    pub static ref TILE_CODE_MAP: HashMap<&'static str, &'static str> = vec![
        ("1_BAMBOO", "1s"),
        ("2_BAMBOO", "2s"),
        ("3_BAMBOO", "3s"),
        ("4_BAMBOO", "4s"),
        ("5_BAMBOO", "5s"),
        ("6_BAMBOO", "6s"),
        ("7_BAMBOO", "7s"),
        ("8_BAMBOO", "8s"),
        ("9_BAMBOO", "9s"),
        ("1_MAN", "1m"),
        ("2_MAN", "2m"),
        ("3_MAN", "3m"),
        ("4_MAN", "4m"),
        ("5_MAN", "5m"),
        ("6_MAN", "6m"),
        ("7_MAN", "7m"),
        ("8_MAN", "8m"),
        ("9_MAN", "9m"),
        ("1_TONG", "1p"),
        ("2_TONG", "2p"),
        ("3_TONG", "3p"),
        ("4_TONG", "4p"),
        ("5_TONG", "5p"),
        ("6_TONG", "6p"),
        ("7_TONG", "7p"),
        ("8_TONG", "8p"),
        ("9_TONG", "9p"),
        ("EAST_WIND", "1z"),
        ("SOUTH_WIND", "2z"),
        ("WEST_WIND", "3z"),
        ("NORTH_WIND", "4z"),
        ("WHITE_DRAGON", "5z"),
        ("GREEN_DRAGON", "6z"),
        ("RED_DRAGON", "7z"),
        ("PLUM_FLOWER", "1f"),
        ("LILY_FLOWER", "2f"),
        ("CHRYSANTHEMUM_FLOWER", "3f"),
        ("BAMBOO_FLOWER", "4f"),
        ("SPRING_SEASON", "5f"),
        ("SUMMER_SEASON", "6f"),
        ("AUTUMN_SEASON", "7f"),
        ("WINTER_SEASON", "8f"),
    ]
    .into_iter()
    .collect();
}
