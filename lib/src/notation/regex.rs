use regex::Regex;

lazy_static! {
    /// Regex to match mpsz format: 11m22p33s44zz5f
    pub static ref TILE_CODE_REGEX: Regex = Regex::new(r"\d+[mpszfj]").unwrap();
}
