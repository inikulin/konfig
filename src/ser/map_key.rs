use super::utils;
use crate::error::{Error, Result};
use serde::ser::{Impossible, Serialize};

pub(crate) struct MapKeySerializer<'o> {
    out: &'o mut String,
}

impl<'o> MapKeySerializer<'o> {
    pub(crate) fn serialize(v: impl Serialize) -> Result<String> {
        let mut key = String::with_capacity(16);

        key.push('[');
        v.serialize(MapKeySerializer { out: &mut key })?;
        key.push(']');

        Ok(key)
    }
}

impl<'o> serde::Serializer for MapKeySerializer<'o> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        utils::write_int(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        utils::write_int(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<()> {
        self.out.push_str(v);

        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::InvalidMapKeyType)
    }
}
