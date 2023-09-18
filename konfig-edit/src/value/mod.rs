mod conv;
mod index;
mod path;
mod to_konfig;

#[cfg(feature = "serde")]
mod to_value;

#[cfg(feature = "serde")]
mod from_value;

#[cfg(feature = "serde")]
mod serde;

pub mod merge;
pub(super) mod value_cell;

use indexmap::IndexMap;

pub use self::path::{Path, PathItem};
pub use self::value_cell::ValueCell;

#[cfg(feature = "serde")]
pub use self::to_value::{to_value, Serializer};

#[cfg(feature = "serde")]
pub use self::from_value::from_value;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    UInt(u64),
    Int(i64),
    Float(f64),
    String(String),
    UnitVariant(String),
    Sequence(Vec<ValueCell>),
    Map(IndexMap<String, ValueCell>),
    Struct(IndexMap<String, ValueCell>),
    Variant(String, ValueCell),
}

impl Value {
    #[inline]
    pub fn into_cell(self) -> ValueCell {
        self.into()
    }
}
