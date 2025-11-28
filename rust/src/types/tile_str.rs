use std::{hash::Hash, marker::PhantomData};

// consider implementing Deref, DerefMut to expose the string directly
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

impl<T> Eq for TileString<T> {}

impl<T> Hash for TileString<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

pub struct TileCodeStruct;

pub type TileCode = TileString<TileCodeStruct>;
