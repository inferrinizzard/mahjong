use std::ops::{Deref, DerefMut};

/// mpsz format input eg. 11m22p33s44zz5f
pub struct TileString {
    pub value: String,
}

impl TileString {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl From<String> for TileString {
    fn from(value: String) -> Self {
        TileString::new(value)
    }
}

impl Deref for TileString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DerefMut for TileString {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.value
    }
}
