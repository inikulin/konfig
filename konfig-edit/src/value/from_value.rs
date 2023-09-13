use super::{Value, ValueCell};
use crate::error::{Error, Result};
use serde::de::IntoDeserializer;
use serde::de::Unexpected;
use serde::de::{Deserialize, DeserializeSeed, Visitor};
use serde::forward_to_deserialize_any;
use std::collections::hash_map::{self, HashMap};
use std::vec;

pub fn from_value<'a, T>(value: Value) -> Result<T>
where
    T: Deserialize<'a>,
{
    T::deserialize(value)
}

impl<'de> serde::de::Deserializer<'de> for Value {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(v),
            Value::Float(v) => visitor.visit_f64(v),
            Value::Int(v) => visitor.visit_i64(v),
            Value::UInt(v) => visitor.visit_u64(v),
            Value::String(v) => visitor.visit_string(v),
            Value::Sequence(v) => deserialize_seq(v, visitor),
            Value::Map(v) | Value::Struct(v) => deserialize_map(v, visitor),
            Value::UnitVariant(v) => visitor.visit_enum(EnumDeserializer {
                variant: v,
                value: None,
            }),
            Value::Variant(v, value) => visitor.visit_enum(EnumDeserializer {
                variant: v,
                value: Some(value),
            }),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    forward_to_deserialize_any! {
        bool
        u8 u16 u32 u64
        i8 i16 i32 i64
        f32 f64
        char str string
        bytes byte_buf
        unit
        unit_struct tuple_struct struct
        seq tuple
        map
        enum
        identifier
        ignored_any
    }
}

fn deserialize_seq<'de, V>(elems: Vec<ValueCell>, visitor: V) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    let len = elems.len();

    let mut deserializer = SeqDeserializer {
        len,
        iter: elems.into_iter(),
    };

    let seq = visitor.visit_seq(&mut deserializer)?;

    if deserializer.iter.len() == 0 {
        Ok(seq)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in sequence",
        ))
    }
}

fn deserialize_map<'de, V>(elems: HashMap<String, ValueCell>, visitor: V) -> Result<V::Value>
where
    V: Visitor<'de>,
{
    let len = elems.len();

    let mut deserializer = MapDeserializer {
        len,
        iter: elems.into_iter(),
        next_value: None,
    };

    let map = visitor.visit_map(&mut deserializer)?;

    if deserializer.iter.len() == 0 {
        Ok(map)
    } else {
        Err(serde::de::Error::invalid_length(
            len,
            &"fewer elements in map",
        ))
    }
}

struct SeqDeserializer {
    len: usize,
    iter: vec::IntoIter<ValueCell>,
}

impl<'de> serde::de::SeqAccess<'de> for SeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value.into_value()).map(Some),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapDeserializer {
    len: usize,
    iter: hash_map::IntoIter<String, ValueCell>,
    next_value: Option<ValueCell>,
}

impl<'de> serde::de::MapAccess<'de> for MapDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.next_value = Some(value);
                seed.deserialize(MapKeyDeserializer { key }).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            Some(value) => seed.deserialize(value.into_value()),
            None => Err(serde::de::Error::custom("value is missing")),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

struct MapKeyDeserializer {
    key: String,
}

macro_rules! deserialize_with_from_str {
    ( $( $de_fn:ident => $vis_fn:ident ),+ ) => {
        $(
            fn $de_fn<V>(self, visitor: V) -> Result<V::Value>
            where
                V: Visitor<'de>,
            {
                visitor.$vis_fn(self.key.parse().map_err(|e| serde::de::Error::custom(e))?)
            }
        )+
    };
}

impl<'de> serde::de::Deserializer<'de> for MapKeyDeserializer {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.key)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // NOTE: keys cannot be null.
        visitor.visit_some(self)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.key
            .into_deserializer()
            .deserialize_enum(name, variants, visitor)
    }

    deserialize_with_from_str! {
        deserialize_i8 => visit_i8,
        deserialize_i16 => visit_i16,
        deserialize_i32 => visit_i32,
        deserialize_i64 => visit_i64,

        deserialize_u8 => visit_u8,
        deserialize_u16 => visit_u16,
        deserialize_u32 => visit_u32,
        deserialize_u64 => visit_u64,

        deserialize_bool => visit_bool
    }

    forward_to_deserialize_any! {
        f32 f64
        char str string
        bytes byte_buf
        unit
        unit_struct tuple_struct struct
        seq tuple
        map
        identifier
        ignored_any
    }
}

struct EnumDeserializer {
    variant: String,
    value: Option<ValueCell>,
}

impl<'de> serde::de::EnumAccess<'de> for EnumDeserializer {
    type Error = Error;
    type Variant = EnumVariantDeserializer;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(self.variant.into_deserializer())?;
        let deserializer = EnumVariantDeserializer { value: self.value };

        Ok((variant, deserializer))
    }
}

struct EnumVariantDeserializer {
    value: Option<ValueCell>,
}

impl<'de> serde::de::VariantAccess<'de> for EnumVariantDeserializer {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        match self.value {
            Some(_) => Err(serde::de::Error::invalid_type(
                Unexpected::NewtypeVariant,
                &"unit variant",
            )),
            None => Ok(()),
        }
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        match self.value {
            Some(value) => seed.deserialize(value.into_value()),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            )),
        }
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value.map(ValueCell::into_value) {
            Some(Value::Sequence(value)) => deserialize_seq(value, visitor),
            Some(value) => Err(serde::de::Error::invalid_type(
                value.as_unexpected(),
                &"tuple variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"tuple variant",
            )),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value.map(ValueCell::into_value) {
            Some(Value::Struct(value)) => deserialize_map(value, visitor),
            Some(value) => Err(serde::de::Error::invalid_type(
                value.as_unexpected(),
                &"struct variant",
            )),
            None => Err(serde::de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            )),
        }
    }
}

impl Value {
    fn as_unexpected(&self) -> Unexpected {
        match *self {
            Value::Null => Unexpected::Unit,
            Value::Bool(v) => Unexpected::Bool(v),
            Value::Float(v) => Unexpected::Float(v),
            Value::Int(v) => Unexpected::Signed(v),
            Value::UInt(v) => Unexpected::Unsigned(v),
            Value::String(ref v) => Unexpected::Str(v),
            Value::UnitVariant(_) => Unexpected::UnitVariant,
            Value::Sequence(_) => Unexpected::Seq,
            Value::Map(_) | Value::Struct(_) => Unexpected::Map,
            Value::Variant(_, ref v) => match **v {
                Value::Struct(_) => Unexpected::StructVariant,
                Value::Sequence(_) => Unexpected::TupleVariant,
                _ => Unexpected::NewtypeVariant,
            },
        }
    }
}
