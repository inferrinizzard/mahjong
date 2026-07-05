use regex::Regex;

lazy_static! {
    pub static ref SINGLE_TILE_CODE_REGEX: Regex = Regex::new(r"\d+\w").unwrap();
    pub static ref TILE_CODE_REGEX: Regex = Regex::new(r"(\d+\w)+").unwrap();
}
