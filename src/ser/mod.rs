mod introspector;
mod kv;
mod map_key;
mod seq;
mod utils;

use self::introspector::{Introspector, ValueKind};
use self::kv::{KVSerializer, KVSerializerMode};
use crate::error::{Error, Result};
use seq::SeqSerializer;
use serde::ser::{Impossible, Serialize};
use std::borrow::Cow;

pub struct Serializer<'o> {
    breadcrumbs: Vec<Cow<'static, str>>,
    out: &'o mut String,
    skip_breadcrumbs_serialization: bool,
}

impl<'o> Serializer<'o> {
    pub fn new(out: &'o mut String) -> Self {
        Self {
            breadcrumbs: Default::default(),
            out,
            skip_breadcrumbs_serialization: false,
        }
    }

    #[inline]
    fn push_path(&mut self, key: impl Into<Cow<'static, str>>) {
        self.breadcrumbs.push(key.into());
    }

    #[inline]
    fn push_enum_variant_path(&mut self, variant: &str) {
        self.push_path(format!("`{variant}`"));
    }

    #[inline]
    fn pop_path(&mut self) {
        self.breadcrumbs.pop();
    }

    fn serialize_breadcrumbs(&mut self) {
        if self.skip_breadcrumbs_serialization {
            return;
        }

        if !self.out.is_empty() {
            self.out.push_str("\n\n");
        }

        if self.breadcrumbs.is_empty() {
            self.out.push_str("> ");
        } else {
            for key in &self.breadcrumbs {
                self.out.push_str("> ");
                self.out.push_str(key);
                self.out.push(' ');
            }
        }

        self.out.push_str("= ");
    }
}

impl<'s, 'o> serde::Serializer for &'s mut Serializer<'o> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SeqSerializer<'s, 'o>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = KVSerializer<'s, 'o>;
    type SerializeStruct = KVSerializer<'s, 'o>;
    type SerializeStructVariant = KVSerializer<'s, 'o>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_breadcrumbs();
        self.out.push_str(if v { "true" } else { "false" });

        Ok(())
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

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_breadcrumbs();
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

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_breadcrumbs();
        utils::write_int(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_breadcrumbs();
        utils::write_float(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let mut start = 0;

        self.serialize_breadcrumbs();
        self.out.push('"');

        for (i, c) in v.char_indices() {
            if let Some(esc) = utils::escape_char(c) {
                if start < i {
                    self.out.push_str(&v[start..i]);
                }

                self.out.push_str(esc);
                start = i + 1;
            }
        }

        if start < v.len() {
            self.out.push_str(&v[start..]);
        }

        self.out.push('"');

        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_breadcrumbs();
        self.out.push('`');
        self.out.push_str(variant);
        self.out.push('`');

        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if Introspector::val_kind(value) == ValueKind::Compound {
            self.serialize_unit_variant(name, variant_index, variant)?;
        }

        self.push_enum_variant_path(variant);
        value.serialize(&mut *self)?;
        self.pop_path();

        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.serialize_breadcrumbs();
        self.out.push('[');

        for (i, byte) in v.iter().enumerate() {
            if i > 0 {
                self.out.push_str(", ");
            }

            self.out.push_str(&format!("{byte:#04X?}"))
        }

        self.out.push(']');

        Ok(())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::TuplesUnsupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::TuplesUnsupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::TuplesUnsupported)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(KVSerializer::new(self, KVSerializerMode::Default))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(KVSerializer::new(self, KVSerializerMode::Default))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::StructVariantsUnsupported)
    }
}
