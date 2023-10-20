use super::doc_format::DocWriter;
use super::doc_format::DocsWrittenFlag;
use super::kv::KVSerializer;
use super::seq::SeqSerializer;
use konfig_edit::error::{Error, Result};
use konfig_edit::serializer::components::{write_escaped_str, write_float, write_int};
use konfig_edit::value::Path;
use serde::ser::Serialize;

pub struct Serializer<'o> {
    pub(super) path: Path<'static, DocsWrittenFlag>,
    pub(super) out: &'o mut String,
    pub(super) skip_path_serialization: bool,
    doc_writer: Option<DocWriter>,
}

impl<'o> Serializer<'o> {
    pub fn new(out: &'o mut String, doc_writer: Option<DocWriter>) -> Self {
        Self {
            path: Default::default(),
            out,
            skip_path_serialization: false,
            doc_writer,
        }
    }

    pub(super) fn serialize_path(&mut self) -> Result<()> {
        if self.skip_path_serialization {
            return Ok(());
        }

        if !self.out.is_empty() {
            self.out.push_str("\n\n");
        }

        if let Some(ref doc_writer) = self.doc_writer {
            doc_writer.write_docs_for_path(self.out, &self.path);
        }

        self.path
            .write(self.out)
            .map_err(serde::ser::Error::custom)?;

        self.out.push_str(" = ");

        Ok(())
    }
}

impl<'s, 'o> serde::Serializer for &'s mut Serializer<'o> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = SeqSerializer<'s, 'o>;
    type SerializeTuple = SeqSerializer<'s, 'o>;
    type SerializeTupleStruct = SeqSerializer<'s, 'o>;
    type SerializeTupleVariant = SeqSerializer<'s, 'o>;
    type SerializeMap = KVSerializer<'s, 'o>;
    type SerializeStruct = KVSerializer<'s, 'o>;
    type SerializeStructVariant = KVSerializer<'s, 'o>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.serialize_path()?;
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
        self.serialize_path()?;
        write_int(self.out, v).map_err(serde::ser::Error::custom)
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<()> {
        Err(Error::Int128NotSupported)
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
        self.serialize_path()?;
        write_int(self.out, v).map_err(serde::ser::Error::custom)
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<()> {
        Err(Error::Int128NotSupported)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v.into())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.serialize_path()?;
        write_float(self.out, v)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.serialize_path()?;
        self.out.push('"');
        write_escaped_str(self.out, v).map_err(serde::ser::Error::custom)?;
        self.out.push('"');

        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.serialize_path()?;
        self.out.push_str("null");

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
        self.serialize_path()?;
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
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.path.push_variant_name(variant);
        value.serialize(&mut *self)?;
        self.path.pop();

        Ok(())
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer::new(self))
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        v.serialize(self)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(SeqSerializer::new(self))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(SeqSerializer::new(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.path.push_variant_name(variant);

        Ok(SeqSerializer::new(self))
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
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.path.push_variant_name(variant);

        Ok(KVSerializer::new(self))
    }
}
