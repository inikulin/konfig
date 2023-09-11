use super::{Value, ValueCell};
use crate::ser::utils;
use crate::{Error, Result};
use serde::ser::Impossible;
use serde::Serialize;
use std::collections::HashMap;

pub fn to_value<T>(input: &T) -> Result<Value>
where
    T: Serialize + ?Sized,
{
    input.serialize(Serializer)
}

pub struct Serializer;

impl serde::Serializer for Serializer {
    type Ok = Value;
    type Error = Error;
    type SerializeSeq = ValueSeqSerializer;
    type SerializeTuple = ValueSeqSerializer;
    type SerializeTupleStruct = ValueSeqSerializer;
    type SerializeTupleVariant = ValueTupleVariantSerializer;
    type SerializeMap = ValueMapSerializer;
    type SerializeStruct = ValueMapSerializer;
    type SerializeStructVariant = ValueStructVariantSerializer;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Value> {
        Ok(Value::Bool(v))
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Value> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Value> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Value> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Value> {
        Ok(Value::Int(v))
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<Value> {
        Err(Error::Int128NotSupported)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Value> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Value> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Value> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Value> {
        Ok(Value::UInt(v))
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<Value> {
        Err(Error::Int128NotSupported)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Value> {
        self.serialize_f64(v.into())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Value> {
        if v.is_finite() {
            Ok(Value::Float(v))
        } else {
            Err(Error::InfAndNanNotSupported)
        }
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Value> {
        Ok(Value::String(v.to_string()))
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Value> {
        Ok(Value::String(v.to_string()))
    }

    #[inline]
    fn serialize_none(self) -> Result<Value> {
        Ok(Value::Null)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Value> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Value> {
        self.serialize_none()
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Value> {
        Ok(Value::UnitVariant(variant.to_string()))
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Value>
    where
        T: ?Sized + Serialize,
    {
        Ok(Value::Variant(
            variant.to_string(),
            value.serialize(self)?.into(),
        ))
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(ValueSeqSerializer {
            items: Vec::with_capacity(len.unwrap_or(0)),
        })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Value> {
        Ok(Value::Sequence(
            v.iter().map(|&v| Value::UInt(v.into()).into()).collect(),
        ))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(ValueTupleVariantSerializer {
            variant: variant.to_string(),
            seq_serializer: ValueSeqSerializer {
                items: Vec::with_capacity(len),
            },
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(ValueMapSerializer {
            items: HashMap::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(ValueStructVariantSerializer {
            variant: variant.to_string(),
            struct_serializer: ValueMapSerializer {
                items: HashMap::with_capacity(len),
                next_key: None,
            },
        })
    }
}

pub struct ValueSeqSerializer {
    items: Vec<ValueCell>,
}

impl serde::ser::SerializeSeq for ValueSeqSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.items.push(to_value(value)?.into());

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value> {
        Ok(Value::Sequence(self.items))
    }
}

impl serde::ser::SerializeTuple for ValueSeqSerializer {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl serde::ser::SerializeTupleStruct for ValueSeqSerializer {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value> {
        serde::ser::SerializeSeq::end(self)
    }
}

pub struct ValueTupleVariantSerializer {
    variant: String,
    seq_serializer: ValueSeqSerializer,
}

impl serde::ser::SerializeTupleVariant for ValueTupleVariantSerializer {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeTuple::serialize_element(&mut self.seq_serializer, value)
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Variant(
            self.variant,
            serde::ser::SerializeTuple::end(self.seq_serializer)?.into(),
        ))
    }
}

pub struct ValueMapSerializer {
    items: HashMap<String, ValueCell>,
    next_key: Option<String>,
}

impl serde::ser::SerializeMap for ValueMapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.serialize(ValueMapKeySerializer)?);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.items
            .insert(self.next_key.take().unwrap(), to_value(value)?.into());

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Value> {
        Ok(Value::Map(self.items))
    }
}

impl serde::ser::SerializeStruct for ValueMapSerializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.next_key = Some(key.to_string());

        serde::ser::SerializeMap::serialize_value(self, value)
    }

    #[inline]
    fn end(self) -> Result<Value> {
        Ok(Value::Struct(self.items))
    }
}

pub struct ValueStructVariantSerializer {
    variant: String,
    struct_serializer: ValueMapSerializer,
}

impl serde::ser::SerializeStructVariant for ValueStructVariantSerializer {
    type Ok = Value;
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeStruct::serialize_field(&mut self.struct_serializer, key, value)
    }

    fn end(self) -> Result<Value> {
        Ok(Value::Variant(
            self.variant,
            serde::ser::SerializeStruct::end(self.struct_serializer)?.into(),
        ))
    }
}

pub struct ValueMapKeySerializer;

impl serde::Serializer for ValueMapKeySerializer {
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<String, Error>;
    type SerializeTuple = Impossible<String, Error>;
    type SerializeTupleStruct = Impossible<String, Error>;
    type SerializeTupleVariant = Impossible<String, Error>;
    type SerializeMap = Impossible<String, Error>;
    type SerializeStruct = Impossible<String, Error>;
    type SerializeStructVariant = Impossible<String, Error>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<String> {
        Ok(v.to_string())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<String> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<String> {
        self.serialize_i64(v.into())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<String> {
        self.serialize_i64(v.into())
    }

    fn serialize_i64(self, v: i64) -> Result<String> {
        let mut out = String::with_capacity(16);

        utils::write_int(&mut out, v);

        Ok(out)
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<String> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<String> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<String> {
        self.serialize_u64(v.into())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<String> {
        let mut out = String::with_capacity(16);

        utils::write_int(&mut out, v);

        Ok(out)
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<String> {
        Ok(v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<String> {
        Ok(v.to_string())
    }

    #[inline]
    fn serialize_none(self) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        Err(Error::InvalidMapKeyType)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<String> {
        Ok(variant.to_string())
    }

    #[inline]
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
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
    fn serialize_bytes(self, _v: &[u8]) -> Result<String> {
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
