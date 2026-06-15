use std::{num::ParseIntError, string::ParseError};

#[derive(Debug)]
pub struct TileParseError {
    pub message: String,
}
impl TileParseError {
    // pub fn new(message: String) -> Self {
    //     TileParseError { message }
    // }
    pub fn new(message: &str) -> Self {
        // Self::new(message.to_string())
        TileParseError {
            message: message.to_string(),
        }
    }
}

impl From<ParseError> for TileParseError {
    fn from(value: ParseError) -> Self {
        TileParseError {
            message: value.to_string(),
        }
    }
}
impl From<ParseIntError> for TileParseError {
    fn from(value: ParseIntError) -> Self {
        TileParseError {
            message: value.to_string(),
        }
    }
}
