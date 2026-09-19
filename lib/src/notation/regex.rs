use regex::Regex;

lazy_static! {
    pub static ref TILE_CODE_REGEX: Regex = Regex::new(r"\d+\w").unwrap();
}
