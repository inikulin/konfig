use super::{Value, ValueCell};
use crate::parser::parse;
use crate::Error;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::str::FromStr;

impl Value {
    pub fn as_sequence(&self) -> Option<&Vec<ValueCell>> {
        match self {
            Value::Sequence(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_sequence_mut(&mut self) -> Option<&mut Vec<ValueCell>> {
        match self {
            Value::Sequence(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, ValueCell>> {
        match self {
            Value::Map(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&mut HashMap<String, ValueCell>> {
        match self {
            Value::Map(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn as_struct(&self) -> Option<&HashMap<String, ValueCell>> {
        match self {
            Value::Struct(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_struct_mut(&mut self) -> Option<&mut HashMap<String, ValueCell>> {
        match self {
            Value::Struct(ref mut v) => Some(v),
            _ => None,
        }
    }

    pub fn as_variant(&self) -> Option<(&String, &ValueCell)> {
        match self {
            Value::Variant(n, v) => Some((n, v)),
            _ => None,
        }
    }

    pub fn as_variant_mut(&mut self) -> Option<(&mut String, &mut ValueCell)> {
        match self {
            Value::Variant(ref mut n, ref mut v) => Some((n, v)),
            _ => None,
        }
    }

    pub fn as_null(&self) -> Option<()> {
        match self {
            Value::Null => Some(()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Value::UInt(v) => Some(*v),
            Value::Int(v) => (*v).try_into().ok(),
            _ => None,
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            Value::UInt(v) => (*v).try_into().ok(),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            Value::UInt(v) => Some(*v as f64),
            Value::Int(v) => Some(*v as f64),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_unit_variant(&self) -> Option<&str> {
        match self {
            Value::UnitVariant(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    pub fn is_bool(&self) -> bool {
        self.as_bool().is_some()
    }

    pub fn is_u64(&self) -> bool {
        self.as_u64().is_some()
    }

    pub fn is_i64(&self) -> bool {
        self.as_i64().is_some()
    }

    pub fn is_f64(&self) -> bool {
        self.as_f64().is_some()
    }

    pub fn is_string(&self) -> bool {
        self.as_str().is_some()
    }

    pub fn is_unit_variant(&self) -> bool {
        self.as_unit_variant().is_some()
    }

    pub fn is_sequence(&self) -> bool {
        self.as_sequence().is_some()
    }

    pub fn is_map(&self) -> bool {
        self.as_map().is_some()
    }

    pub fn is_struct(&self) -> bool {
        self.as_struct().is_some()
    }

    pub fn is_variant(&self) -> bool {
        self.as_variant().is_some()
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<T>) -> Self {
        Value::Sequence(val.into_iter().map(|v| ValueCell::from(v.into())).collect())
    }
}

impl<K, V> From<BTreeMap<K, V>> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(val: BTreeMap<K, V>) -> Value {
        Value::Map(
            val.into_iter()
                .map(|(k, v)| (k.into(), ValueCell::from(v.into())))
                .collect(),
        )
    }
}

impl<K, V> From<HashMap<K, V>> for Value
where
    K: Into<String> + Hash + Eq,
    V: Into<Value>,
{
    fn from(val: HashMap<K, V>) -> Value {
        Value::Map(
            val.into_iter()
                .map(|(k, v)| (k.into(), ValueCell::from(v.into())))
                .collect(),
        )
    }
}

impl FromStr for Value {
    type Err = Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

macro_rules! impl_from_primitive {
    ( $( [ $($T:ty)+ ] => $variant:ident )+ ) => {
        $(
            $(
                impl From<$T> for Value {
                    #[inline]
                    fn from(v: $T) -> Self {
                        Value::$variant(v.into())
                    }
                }
            )+
        )+
    };
}

impl_from_primitive! {
    [bool] => Bool
    [u8 u16 u32 u64] => UInt
    [i8 i16 i32 i64] => Int
    [f32 f64] => Float
    [String &str] => String
}
