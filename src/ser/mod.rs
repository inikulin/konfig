mod introspector;
mod map_key;
mod utils;

use self::introspector::{Introspector, ValueKind};
use self::map_key::MapKeySerializer;
use crate::error::{Error, Result};
use serde::ser::{Impossible, Serialize};
use std::borrow::Cow;

#[derive(PartialEq, Copy, Clone)]
enum EnumVariantSerializationMode {
    Full,
    AssignmentOnly,
    PayloadOnly,
}

impl EnumVariantSerializationMode {
    #[inline]
    fn serialize_assignment(&self) -> bool {
        *self == Self::Full || *self == Self::AssignmentOnly
    }

    #[inline]
    fn serialize_payload(&self) -> bool {
        *self == Self::Full || *self == Self::PayloadOnly
    }
}

pub struct Serializer<'o> {
    val_path: Vec<Cow<'static, str>>,
    out: &'o mut String,
    skip_val_path_serialization: bool,
    enum_serialization_mode: EnumVariantSerializationMode,
}

impl<'o> Serializer<'o> {
    pub fn new(out: &'o mut String) -> Self {
        Self {
            val_path: vec![],
            out,
            skip_val_path_serialization: false,
            enum_serialization_mode: EnumVariantSerializationMode::Full,
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

    fn with_path_key<T>(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        f: impl Fn(&mut Self) -> T,
    ) -> T {
        self.push_path(key);

        let res = f(self);

        self.pop_path();

        res
    }

    fn without_val_path_serialization<T>(&mut self, f: impl Fn(&mut Self) -> T) -> T {
        self.skip_val_path_serialization = true;

        let res = f(self);

        self.skip_val_path_serialization = false;

        res
    }

    fn serialize_with_output(
        &mut self,
        out: &mut String,
        value: &(impl Serialize + ?Sized),
    ) -> Result<()> {
        // SAFETY: it's safe to extend `out` lifetime here as we don't store the reference for
        // longer than this method call.
        let out: &'o mut String = unsafe { std::mem::transmute(out) };
        let prev_out = std::mem::replace(&mut self.out, out);

        if !self.out.is_empty() {
            self.new_line();
        }

        let res = value.serialize(&mut *self);

        self.out = prev_out;

        res
    }

    fn serialize_enum_variant(
        &mut self,
        out: &mut String,
        mode: EnumVariantSerializationMode,
        value: &(impl Serialize + ?Sized),
    ) -> Result<()> {
        let prev_mode = self.enum_serialization_mode;

        self.enum_serialization_mode = mode;

        let res = self.serialize_with_output(out, value);

        self.enum_serialization_mode = prev_mode;

        res
    }

    fn write_val_path(&mut self) {
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
        self.write_val_path();
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
        self.write_val_path();
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
        self.write_val_path();
        utils::write_int(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_val_path();
        utils::write_float(self.out, v);

        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let mut start = 0;

        self.write_val_path();
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
        self.write_val_path();
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
        self.write_val_path();
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
        if self.enum_serialization_mode.serialize_assignment() {
            self.serialize_unit_variant(name, variant_index, variant)?;
        }

        if self.enum_serialization_mode == EnumVariantSerializationMode::Full {
            self.new_line();
        }

        if self.enum_serialization_mode.serialize_payload() {
            self.with_path_key(variant, |serializer| value.serialize(serializer))?;
        }

        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.write_val_path();
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
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        if self.enum_serialization_mode.serialize_assignment() {
            self.serialize_unit_variant(name, variant_index, variant)?;
        }

        if self.enum_serialization_mode == EnumVariantSerializationMode::Full {
            self.new_line();
        }

        let kv_ser_mode = if self.enum_serialization_mode.serialize_payload() {
            self.push_path(variant);

            KVSerializerMode::WithPathPopOnCompletion
        } else {
            KVSerializerMode::Noop
        };

        Ok(KVSerializer::new(self, kv_ser_mode))
    }
}

enum SeqRepresentation<'s, 'o> {
    // NOTE: wrapped in `Option`, so we can convert to KV without resorting to unsafe code.
    Inline(Option<&'s mut Serializer<'o>>),
    Kv(KVSerializer<'s, 'o>),
}

impl<'s, 'o> SeqRepresentation<'s, 'o> {
    fn serialize_empty(&mut self) {
        let serializer = match self {
            Self::Inline(Some(ref mut s)) => s,
            Self::Kv(s) => &mut *s.serializer,
            _ => unreachable!(),
        };

        serializer.write_val_path();
        serializer.out.push_str("[]");
    }

    fn into_kv(&mut self) {
        let serializer = match self {
            Self::Inline(s) => s.take().expect("should have inline serializer"),
            _ => return,
        };

        *self = SeqRepresentation::Kv(KVSerializer::new(serializer, KVSerializerMode::Default));
    }
}

pub struct SeqSerializer<'s, 'o> {
    repr: SeqRepresentation<'s, 'o>,
    current_index: usize,
}

impl<'s, 'o> SeqSerializer<'s, 'o> {
    fn new(serializer: &'s mut Serializer<'o>) -> Self {
        Self {
            repr: SeqRepresentation::Inline(Some(serializer)),
            current_index: 0,
        }
    }
}

impl<'s, 'o> serde::ser::SerializeSeq for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.current_index == 0 && Introspector::val_kind(value) != ValueKind::Leaf {
            self.repr.into_kv();
        }

        match self.repr {
            SeqRepresentation::Kv(ref mut serializer) => {
                serde::ser::SerializeMap::serialize_key(serializer, &self.current_index)?;
                serde::ser::SerializeMap::serialize_value(serializer, value)?;
            }
            SeqRepresentation::Inline(Some(ref mut serializer)) => {
                if self.current_index == 0 {
                    serializer.write_val_path();
                    serializer.out.push('[');
                } else {
                    serializer.out.push_str(", ");
                }

                serializer
                    .without_val_path_serialization(|serializer| value.serialize(serializer))?;
            }
            _ => unreachable!(),
        }

        self.current_index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<()> {
        if self.current_index == 0 {
            self.repr.serialize_empty();
        } else {
            match self.repr {
                SeqRepresentation::Kv(serializer) => serde::ser::SerializeStruct::end(serializer)?,
                SeqRepresentation::Inline(Some(serializer)) => serializer.out.push(']'),
                _ => unreachable!(),
            }
        }

        Ok(())
    }
}

#[derive(PartialEq)]
enum KVSerializerMode {
    Default,
    WithPathPopOnCompletion,
    Noop,
}

pub struct KVSerializer<'s, 'o> {
    serializer: &'s mut Serializer<'o>,
    leaf_values_out: String,
    enum_values_out: String,
    compound_values_out: String,
    mode: KVSerializerMode,
}

impl<'s, 'o> KVSerializer<'s, 'o> {
    fn new(serializer: &'s mut Serializer<'o>, mode: KVSerializerMode) -> Self {
        Self {
            serializer,
            leaf_values_out: "".into(),
            enum_values_out: "".into(),
            compound_values_out: "".into(),
            mode,
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
        if self.mode != KVSerializerMode::Noop {
            self.serializer.push_path(MapKeySerializer::serialize(key)?);
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        match Introspector::val_kind(value) {
            ValueKind::Leaf | ValueKind::KvOnlyLeaf => self
                .serializer
                .serialize_with_output(&mut self.leaf_values_out, value)?,
            ValueKind::NonUnitEnumVariant => {
                // NOTE: serialize assignment of the variant along with the rest of leaf values,
                // but move payload to the compound values block.
                self.serializer.serialize_enum_variant(
                    &mut self.leaf_values_out,
                    EnumVariantSerializationMode::AssignmentOnly,
                    value,
                )?;

                self.serializer.serialize_enum_variant(
                    &mut self.enum_values_out,
                    EnumVariantSerializationMode::PayloadOnly,
                    value,
                )?;
            }
            _ => self
                .serializer
                .serialize_with_output(&mut self.compound_values_out, value)?,
        }

        self.serializer.pop_path();

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        self.serializer.out.push_str(&self.leaf_values_out);

        if !self.leaf_values_out.is_empty() && !self.enum_values_out.is_empty() {
            self.serializer.new_line();
        }

        self.serializer.out.push_str(&self.enum_values_out);

        if !self.enum_values_out.is_empty() && !self.compound_values_out.is_empty() {
            self.serializer.new_line();
        }

        self.serializer.out.push_str(&self.compound_values_out);

        if self.mode == KVSerializerMode::WithPathPopOnCompletion {
            self.serializer.pop_path();
        }

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
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        self.serializer.push_path(key);

        serde::ser::SerializeMap::serialize_value(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        serde::ser::SerializeMap::end(self)
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
        serde::ser::SerializeMap::end(self)
    }
}
