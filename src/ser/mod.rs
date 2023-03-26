mod basic_value;
mod map_key;
mod utils;

use crate::error::{Error, Result};
use basic_value::is_basic_value;
use serde::ser::{Impossible, Serialize};
use std::borrow::Cow;

pub struct Serializer<'o> {
    val_path: Vec<Cow<'static, str>>,
    out: &'o mut String,
    skip_val_path_serialization: bool,
}

impl<'o> Serializer<'o> {
    pub fn new(out: &'o mut String) -> Self {
        Self {
            val_path: vec![],
            out,
            skip_val_path_serialization: false,
        }
    }

    #[inline]
    fn new_line(&mut self) {
        self.out.push_str("\n\n");
    }

    #[inline]
    fn push_path(&mut self, key: impl Into<Cow<'static, str>>) {
        self.val_path.push(key.into());
    }

    #[inline]
    fn pop_path(&mut self) {
        self.val_path.pop();
    }

    fn serialize_val_path(&mut self) {
        if self.skip_val_path_serialization {
            return;
        }

        if self.val_path.is_empty() {
            self.out.push_str("> ");
        } else {
            for key in &self.val_path {
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
        self.serialize_val_path();
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
        self.serialize_val_path();
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
        self.serialize_val_path();
        utils::write_int(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_val_path();
        utils::write_float(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let mut start = 0;

        self.serialize_val_path();
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
        self.serialize_val_path();
        self.out.push_str("none");

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
        self.serialize_val_path();
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
        self.serialize_unit_variant(name, variant_index, variant)?;
        self.push_path(variant);
        self.new_line();

        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.serialize_val_path();
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
        Ok(KVSerializer::new(self))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(KVSerializer::new(self))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_unit_variant(name, variant_index, variant)?;
        self.push_path(variant);
        self.new_line();

        Ok(KVSerializer::new(self))
    }
}

pub struct SeqSerializer<'s, 'o> {
    serializer: &'s mut Serializer<'o>,
    current_index: usize,
    is_basic_values: bool,
}

impl<'s, 'o> SeqSerializer<'s, 'o> {
    fn new(serializer: &'s mut Serializer<'o>) -> Self {
        Self {
            serializer,
            current_index: 0,
            is_basic_values: false,
        }
    }

    fn serialize_basic_value_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.current_index == 0 {
            self.serializer.serialize_val_path();
            self.serializer.out.push('[');
        } else {
            self.serializer.out.push_str(", ");
        }

        self.serializer.skip_val_path_serialization = true;

        value.serialize(&mut *self.serializer)?;

        self.serializer.skip_val_path_serialization = false;

        Ok(())
    }

    fn serialize_compound_value_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.current_index > 0 {
            self.serializer.new_line();
        }

        self.serializer
            .push_path(format!("[{}]", self.current_index));

        value.serialize(&mut *self.serializer)?;

        self.serializer.pop_path();

        Ok(())
    }

    fn serialize_empty(&mut self) {
        self.serializer.serialize_val_path();
        self.serializer.out.push_str("[]");
    }
}

impl<'s, 'o> serde::ser::SerializeSeq for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.current_index == 0 {
            self.is_basic_values = is_basic_value(value);
        }

        if self.is_basic_values {
            self.serialize_basic_value_element(value)?;
        } else {
            self.serialize_compound_value_element(value)?;
        }

        self.current_index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<()> {
        let was_empty = self.current_index == 0;

        if was_empty {
            self.serialize_empty();
        } else if self.is_basic_values {
            self.serializer.out.push(']');
        }

        Ok(())
    }
}

pub struct KVSerializer<'s, 'o> {
    serializer: &'s mut Serializer<'o>,
    current_index: usize,
}

impl<'s, 'o> KVSerializer<'s, 'o> {
    fn new(serializer: &'s mut Serializer<'o>) -> Self {
        Self {
            serializer,
            current_index: 0,
        }
    }
}

impl<'s, 'o> serde::ser::SerializeMap for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.push_path(map_key::serialize(key)?);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.current_index > 0 {
            self.serializer.new_line();
        }

        value.serialize(&mut *self.serializer)?;
        self.serializer.pop_path();

        self.current_index += 1;

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'s, 'o> serde::ser::SerializeStruct for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serializer.push_path(key);
        serde::ser::SerializeMap::serialize_value(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'s, 'o> serde::ser::SerializeStructVariant for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        Ok(())
    }
}
