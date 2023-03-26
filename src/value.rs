use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Value {
    None,
    Bool(bool),
    SignedInt(i64),
    UnsignedInt(u64),
    Float(u64),
    String(String),
    Vec(Vec<Value>),
    Struct(IndexMap<String, Value>),
    Map(IndexMap<String, Value>),
    EnumVariant {
        name: String,
        value: Box<Value>,
    },
}

impl Default for Value {
    fn default() -> Self {
        Self::None
    }
}
