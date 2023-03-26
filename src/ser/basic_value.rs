use serde::ser::{Impossible, Serialize};

#[derive(thiserror::Error, Debug)]
#[error("")]
struct ValueIsCompound;

impl serde::ser::Error for ValueIsCompound {
    fn custom<T>(_msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self
    }
}

#[inline]
pub(crate) fn is_basic_value(val: impl Serialize) -> bool {
    val.serialize(IsBasicValue).is_ok()
}

struct IsBasicValue;

impl serde::Serializer for IsBasicValue {
    type Ok = ();
    type Error = ValueIsCompound;
    type SerializeSeq = Impossible<(), ValueIsCompound>;
    type SerializeTuple = Impossible<(), ValueIsCompound>;
    type SerializeTupleStruct = Impossible<(), ValueIsCompound>;
    type SerializeTupleVariant = Impossible<(), ValueIsCompound>;
    type SerializeMap = Impossible<(), ValueIsCompound>;
    type SerializeStruct = Impossible<(), ValueIsCompound>;
    type SerializeStructVariant = Impossible<(), ValueIsCompound>;

    #[inline]
    fn serialize_bool(self, _v: bool) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, _v: i8) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, _v: i16) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, _v: i32) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, _v: i64) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, _v: u8) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, _v: u16) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, _v: u32) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, _v: u64) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, _v: f32) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, _v: f64) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_char(self, _v: char) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_str(self, _v: &str) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<(), ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_some<T>(self, _value: &T) -> Result<(), ValueIsCompound>
    where
        T: ?Sized + Serialize,
    {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_unit(self) -> Result<(), ValueIsCompound> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), ValueIsCompound>
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
    ) -> Result<(), ValueIsCompound>
    where
        T: ?Sized + Serialize,
    {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_bytes(self, _v: &[u8]) -> Result<(), ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, ValueIsCompound> {
        Err(ValueIsCompound)
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, ValueIsCompound> {
        Err(ValueIsCompound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Unit;

    #[derive(Serialize, Default)]
    struct Tuple(usize, usize);

    #[derive(Serialize)]
    enum Variants {
        Unit,
        Tuple(usize, usize),
        Struct { foo: usize },
    }

    #[derive(Serialize, Default)]
    struct NewTypeStruct<T: Serialize + Default>(T);

    #[derive(Serialize, Default)]
    struct Struct {
        foo: usize,
        bar: usize,
    }

    macro_rules! assert_basic {
        ( $($Ty:ty),+ ) => {
            $(
                assert!(is_basic_value(<$Ty>::default()));
            )+
        };
    }

    macro_rules! assert_non_basic {
        ( $($val:expr),+ ) => {
            $(
                assert!(!is_basic_value($val));
            )+
        };
    }

    #[test]
    fn basic_values() {
        assert_basic! {
            i8, i16, i32, i64, isize,
            u8, u16, u32, u64, usize,
            f32, f64, char, bool, (),
            String, &str,
            NewTypeStruct<bool>
        }
    }

    #[test]
    fn non_basic_values() {
        assert_non_basic! {
            None::<()>,
            Some(42),
            Ok::<usize, ()>(50),
            vec![1,2,3],
            (1, 2, 3),
            std::collections::BTreeMap::<String, String>::default(),
            0i128, 0u128,
            Unit,
            Variants::Unit,
            Variants::Tuple(0, 0),
            Variants::Struct { foo: 0 },
            NewTypeStruct(vec![0]),
            std::ffi::CString::default(),
            Tuple::default(),
            Struct::default()
        }
    }
}
