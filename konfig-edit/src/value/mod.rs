mod conv;
mod index;

#[cfg(feature = "serde")]
mod to_value;

#[cfg(feature = "serde")]
mod from_value;

#[cfg(feature = "serde")]
mod serde;

pub(super) mod value_cell;

use std::collections::HashMap;

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
    Map(HashMap<String, ValueCell>),
    Struct(HashMap<String, ValueCell>),
    Variant(String, ValueCell),
}
