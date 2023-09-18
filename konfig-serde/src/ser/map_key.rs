use konfig_edit::error::{Error, Result};
use serde::ser::{Impossible, Serialize};
use std::borrow::Cow;

pub(crate) struct MapKeySerializer;

impl serde::Serializer for MapKeySerializer {
    type Ok = Cow<'static, str>;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Error>;
    type SerializeTuple = Impossible<Self::Ok, Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Error>;
    type SerializeMap = Impossible<Self::Ok, Error>;
    type SerializeStruct = Impossible<Self::Ok, Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(if v { "true" } else { "false" }.into())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Ok(v.to_string().into())
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Ok(v.to_string().into())
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.to_string().into())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(variant.into())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
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
    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
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
