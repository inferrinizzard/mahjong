use std::marker::PhantomData;

pub struct TileString<T> {
    pub value: String,
    _type: PhantomData<T>,
}

impl<T> TileString<T> {
    pub fn new(value: String) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }
}

impl<T> From<&str> for TileString<T> {
    fn from(value: &str) -> Self {
        TileString::new(String::from(value))
    }
}

impl<T> From<String> for TileString<T> {
    fn from(value: String) -> Self {
        TileString::new(value)
    }
}

impl<T> Default for TileString<T> {
    fn default() -> Self {
        TileString::new(String::new())
    }
}

impl<T> PartialEq for TileString<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub struct TileCodeStruct;
pub struct TileNameStruct;

pub type TileCode = TileString<TileCodeStruct>;
pub type TileName = TileString<TileNameStruct>;
