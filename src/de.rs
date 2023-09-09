use crate::parse;
use crate::{Error, Result};
use serde::de::{DeserializeOwned, Visitor};

pub fn from_str<T>(input: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    T::deserialize(Deserializer::new(input))
}

pub struct Deserializer<'i> {
    input: &'i str,
}

impl<'i> Deserializer<'i> {
    pub fn new(input: &'i str) -> Self {
        Deserializer { input }
    }
}

macro_rules! impl_simple_deserialize {
    ( $($de_fn:ident)+ ) => {
        $(
            #[inline]
            fn $de_fn<V>(self, visitor: V) -> Result<V::Value>
            where
                V: Visitor<'de>,
            {
                parse(self.input)?.$de_fn(visitor)
            }
        )+
    };
}

impl<'i, 'de> serde::de::Deserializer<'de> for Deserializer<'i> {
    type Error = Error;

    #[inline]
    fn deserialize_unit_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_unit_struct(name, visitor)
    }

    #[inline]
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_tuple(len, visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_newtype_struct(name, visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_tuple_struct(name, len, visitor)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_struct(name, fields, visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        parse(self.input)?.deserialize_enum(name, variants, visitor)
    }

    impl_simple_deserialize! {
        deserialize_any
        deserialize_bool
        deserialize_u8 deserialize_u16 deserialize_u32 deserialize_u64
        deserialize_i8 deserialize_i16 deserialize_i32 deserialize_i64
        deserialize_f32 deserialize_f64
        deserialize_char deserialize_str deserialize_string
        deserialize_bytes deserialize_byte_buf
        deserialize_option deserialize_unit
        deserialize_seq
        deserialize_map
        deserialize_identifier
        deserialize_ignored_any
    }
}
