use serde::ser::{Impossible, Serialize};
use std::convert::Infallible;

#[derive(thiserror::Error, Debug)]
#[error("")]
struct IsPrimitive(bool);

impl serde::ser::Error for IsPrimitive {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self(false)
    }
}

pub(super) fn is_primitive(val: impl Serialize) -> bool {
    match val.serialize(PrimitiveValueChecker) {
        Err(is_primitive) => is_primitive.0,
        Ok(v) => match v {},
    }
}

struct PrimitiveValueChecker;

impl serde::Serializer for PrimitiveValueChecker {
    type Ok = Infallible;
    // NOTE: we return as error to bail early where necessary and avoid implementing
    // additional traits
    type Error = IsPrimitive;
    type SerializeSeq = Impossible<Infallible, IsPrimitive>;
    type SerializeTuple = Impossible<Infallible, IsPrimitive>;
    type SerializeTupleStruct = Impossible<Infallible, IsPrimitive>;
    type SerializeTupleVariant = Impossible<Infallible, IsPrimitive>;
    type SerializeMap = Impossible<Infallible, IsPrimitive>;
    type SerializeStruct = Impossible<Infallible, IsPrimitive>;
    type SerializeStructVariant = Impossible<Infallible, IsPrimitive>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_i128(self, _v: i128) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_u128(self, _v: u128) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_none(self) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_some<T>(self, value: &T) -> Result<Infallible, IsPrimitive>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(true))
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Infallible, IsPrimitive>
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
    ) -> Result<Infallible, IsPrimitive>
    where
        T: ?Sized + Serialize,
    {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<Infallible, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, IsPrimitive> {
        Err(IsPrimitive(false))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, IsPrimitive> {
        Err(IsPrimitive(false))
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

    #[derive(Serialize, Default)]
    struct TupleCompound(usize, Vec<usize>);

    #[derive(Serialize)]
    enum Variants {
        Unit,
        Tuple(usize, usize),
        TupleCompound(usize, Vec<usize>),
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
    fn primitve_values() {
        macro_rules! assert_primitive {
            ( $($val:expr),+ ) => {
                $(
                    assert!(is_primitive($val));
                )+
            };
        }

        assert_primitive! {
            0i8, 0i16, 0i32, 0i64, 0isize,
            0u8, 0u16, 0u32, 0u64, 0usize,
            0f32, 0f64, 'k', true, (),
            String::from("foobar"), "foobar",
            NewTypeStruct(false),
            Unit,
            None::<String>,
            Variants::Unit
        }
    }

    #[test]
    fn non_primitive_values() {
        macro_rules! assert_not_primitive {
            ( $($val:expr),+ ) => {
                $(
                    assert!(!is_primitive($val));
                )+
            };
        }

        assert_not_primitive! {
            vec![vec![1], vec![2], vec![3]],
            std::collections::BTreeMap::<String, String>::default(),
            Struct::default(),
            Variants::Struct { foo: 123 },
            Some(vec![vec![1], vec![2], vec![3]]),
            (1, 2, vec![1]),
            TupleCompound::default(),
            Variants::TupleCompound(1, vec![1]),
            vec![1, 2, 3],
            Vec::<String>::new(),
            std::ffi::CString::default(),
            Variants::NewType(0),
            (1, 2, false),
            Tuple::default(),
            Variants::Tuple(0, 0),
            0i128,
            0u128
        }
    }
}
