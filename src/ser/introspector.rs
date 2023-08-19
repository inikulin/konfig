use serde::ser::{Impossible, Serialize};
use std::convert::Infallible;

#[derive(Clone, Copy, thiserror::Error, Debug, PartialEq)]
#[error("")]
pub(crate) enum ValueKind {
    Leaf,
    KvOnlyLeaf,
    Compound,
    Unsupported,
}

impl serde::ser::Error for ValueKind {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Unsupported
    }
}

pub(crate) struct Introspector;

impl Introspector {
    #[inline]
    pub(crate) fn val_kind(val: impl Serialize) -> ValueKind {
        match val.serialize(Introspector) {
            Err(info) => info,
            Ok(v) => match v {},
        }
    }
}

impl serde::Serializer for Introspector {
    type Ok = Infallible;
    // NOTE: we return as error to bail early where necessary and avoid implementing
    // additional traits
    type Error = ValueKind;
    type SerializeSeq = Self;
    type SerializeTuple = Impossible<Infallible, ValueKind>;
    type SerializeTupleStruct = Impossible<Infallible, ValueKind>;
    type SerializeTupleVariant = Impossible<Infallible, ValueKind>;
    type SerializeMap = Impossible<Infallible, ValueKind>;
    type SerializeStruct = Impossible<Infallible, ValueKind>;
    type SerializeStructVariant = Impossible<Infallible, ValueKind>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Unsupported)
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Unsupported)
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_none(self) -> Result<Infallible, ValueKind> {
        Err(ValueKind::KvOnlyLeaf)
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Infallible, ValueKind>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Infallible, ValueKind> {
        Err(ValueKind::Leaf)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Infallible, ValueKind> {
        Err(ValueKind::KvOnlyLeaf)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Infallible, ValueKind>
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
        value: &T,
    ) -> Result<Infallible, ValueKind>
    where
        T: ?Sized + Serialize,
    {
        let val_kind = value.serialize(self);

        // NOTE: we can't inline newtype enum variants in sequences as this will erase enum
        // variant information as we don't emit breadcrumbs for those, e.g.:
        // `> [0] > `Foo` = 123` will be inlined as `> = [ 123 ]`
        if let Err(ValueKind::Leaf) = val_kind {
            return Err(ValueKind::KvOnlyLeaf);
        }

        val_kind
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, ValueKind> {
        Ok(Self)
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Infallible, ValueKind> {
        Err(ValueKind::KvOnlyLeaf)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, ValueKind> {
        Err(ValueKind::Unsupported)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, ValueKind> {
        Err(ValueKind::Unsupported)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, ValueKind> {
        Err(ValueKind::Unsupported)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, ValueKind> {
        Err(ValueKind::Compound)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, ValueKind> {
        Err(ValueKind::Compound)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, ValueKind> {
        Err(ValueKind::Unsupported)
    }
}

impl serde::ser::SerializeSeq for Introspector {
    type Ok = Infallible;
    type Error = ValueKind;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), ValueKind>
    where
        T: ?Sized + Serialize,
    {
        // NOTE: we can't have nested sequences in leaf values (e.g. `[[], []]`),
        // but we can have sequence with leaf values (e.g. `[1, 2, 3]` ) as
        // a leaf structure field
        Err(if Introspector::val_kind(value) == ValueKind::Leaf {
            ValueKind::KvOnlyLeaf
        } else {
            ValueKind::Compound
        })
    }

    #[inline]
    fn end(self) -> Result<Infallible, ValueKind> {
        // NOTE: we can have `[]` as a leaf structure field
        Err(ValueKind::KvOnlyLeaf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize, Default)]
    struct Unit;

    #[derive(Serialize, Default)]
    struct Tuple(usize, usize);

    #[derive(Serialize)]
    enum Variants {
        Unit,
        Tuple(usize, usize),
        Struct { foo: usize },
        NewType(usize),
    }

    #[derive(Serialize, Default)]
    struct NewTypeStruct<T: Serialize + Default>(T);

    #[derive(Serialize, Default)]
    struct Struct {
        foo: usize,
        bar: usize,
    }

    #[test]
    fn leaf_values() {
        macro_rules! assert_leaf {
            ( $($Ty:ty),+ ) => {
                $(
                    assert_eq!(Introspector::val_kind(<$Ty>::default()), ValueKind::Leaf);
                )+
            };
        }

        assert_leaf! {
            i8, i16, i32, i64, isize,
            u8, u16, u32, u64, usize,
            f32, f64, char, bool, (),
            String, &str,
            NewTypeStruct<bool>,
            Unit
        }

        assert_eq!(Introspector::val_kind(Some(42u8)), ValueKind::Leaf);
    }

    #[test]
    fn compound_values() {
        macro_rules! assert_compound {
            ( $($val:expr),+ ) => {
                $(
                    assert_eq!(Introspector::val_kind($val), ValueKind::Compound);
                )+
            };
        }

        assert_compound! {
            vec![vec![1], vec![2], vec![3]],
            std::collections::BTreeMap::<String, String>::default(),
            Struct::default(),
            Some(vec![vec![1], vec![2], vec![3]])
        }
    }

    #[test]
    fn kv_only_leaf_values() {
        macro_rules! assert_kv_only_leaf {
            ( $($val:expr),+ ) => {
                $(
                    assert_eq!(Introspector::val_kind($val), ValueKind::KvOnlyLeaf);
                )+
            };
        }

        assert_kv_only_leaf! {
            None::<String>,
            Variants::Unit,
            vec![1, 2, 3],
            Vec::<String>::new(),
            std::ffi::CString::default(),
            Variants::NewType(0)
        }
    }

    #[test]
    fn unsupported_value() {
        macro_rules! assert_unsupported {
            ( $($val:expr),+ ) => {
                $(
                    assert_eq!(Introspector::val_kind($val), ValueKind::Unsupported);
                )+
            };
        }

        assert_unsupported! {
            0i128,
            0u128,
            Variants::Tuple(0, 0),
            Variants::Struct { foo: 123 },
            (1, 2, false),
            Tuple::default()
        }
    }
}
