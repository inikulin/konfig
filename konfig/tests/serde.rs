use indoc::indoc;
use konfig::error::Error;
use konfig::{Value, ValueCell};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ffi::CString;

macro_rules! ok {
    ($rust:expr => $kfg:expr $(, expected_to_value: $expected_to_value:expr)?) => {
        assert_eq!(
            konfig::to_string(&$rust),
            Ok($kfg.to_string()),
            "serialize Rust value"
        );

        #[allow(unused_mut, unused_assignments)]
        let mut expected_to_value = konfig::parse($kfg).map(Into::into);

        // NOTE: sometimes serialization and unhinted parsing are not roundtripable,
        // e.g. serialized positive `i64` will be parsed as `u64`. For these cases we
        // can't obtain expected `Value` by parsing seriazlied value with `konfig:parse`
        // and need to use this hint instead.
        $(
            expected_to_value = Ok($expected_to_value);
        )?

        let value = konfig::to_value(&$rust);

        assert_eq!(value, expected_to_value, "to_value");

        let value = value.unwrap();

        assert_eq!(ValueCell::from(value.clone()).to_konfig(), Ok($kfg.to_string()), "to_konfig");
        assert_eq!(konfig::from_value(value), Ok($rust), "from_value");
        assert_eq!(konfig::from_str($kfg), Ok($rust), "deserialize to Rust value");
    };
}

macro_rules! ser_err {
    ($rust:expr => $err:expr) => {
        assert_eq!(
            konfig::to_string(&$rust),
            Err($err),
            "serialize Rust value should error"
        );

        assert_eq!(konfig::to_value(&$rust), Err($err), "to_value should error");
    };
}

macro_rules! map {
    ($($k:expr => $v:expr),+) => {
        [$(($k.clone(), $v.clone())),+].into_iter().collect::<BTreeMap<_, _>>()
    };
}

#[test]
fn bool_val() {
    ok! { true => "> = true" }
    ok! { false => "> = false" }
}

#[test]
fn i8_val() {
    ok! {
        42i8 => "> = 42",
        expected_to_value: Value::Int(42)
    }

    ok! { -42i8 => "> = -42" }
    ok! { i8::MIN => "> = -128" }

    ok! {
        i8::MAX => "> = 127",
        expected_to_value: Value::Int(127)
    }
}

#[test]
fn i16_val() {
    ok! {
        42i16 => "> = 42",
        expected_to_value: Value::Int(42)
    }

    ok! { -42i16 => "> = -42" }
    ok! { i16::MIN => "> = -32768" }

    ok! {
        i16::MAX => "> = 32767",
        expected_to_value: Value::Int(32767)
    }
}

#[test]
fn i32_val() {
    ok! {
        42i32 => "> = 42",
        expected_to_value: Value::Int(42)
    }

    ok! { -42i32 => "> = -42" }
    ok! { i32::MIN => "> = -2147483648" }

    ok! {
        i32::MAX => "> = 2147483647",
        expected_to_value: Value::Int(2147483647)
    }
}

#[test]
fn i64_val() {
    ok! {
        3i64 => "> = 3",
        expected_to_value: Value::Int(3)
    }

    ok! { -2i64 => "> = -2" }
    ok! { -1234i64 => "> = -1234" }
    ok! { i64::MIN => "> = -9223372036854775808" }

    ok! {
        i64::MAX => "> = 9223372036854775807",
        expected_to_value: Value::Int(9223372036854775807)
    }
}

#[test]
fn i128_val() {
    ser_err! { 0i128 => Error::Int128NotSupported }
}

#[test]
fn u8_val() {
    ok! { 42u8 => "> = 42" }
    ok! { u8::MIN => "> = 0" }
    ok! { u8::MAX => "> = 255" }
}

#[test]
fn u16_val() {
    ok! { 42u16 => "> = 42" }
    ok! { u16::MIN => "> = 0" }
    ok! { u16::MAX => "> = 65535" }
}

#[test]
fn u32_val() {
    ok! { 1337u32 => "> = 1337" }
    ok! { u32::MIN => "> = 0" }
    ok! { u32::MAX => "> = 4294967295" }
}

#[test]
fn u64_val() {
    ok! { 1337u64 => "> = 1337" }
    ok! { u64::MIN => "> = 0" }
    ok! { u64::MAX => "> = 18446744073709551615" }
}

#[test]
fn u128_val() {
    ser_err! { 0u128 => Error::Int128NotSupported }
}

#[test]
fn f32_val() {
    // NOTE: f32 looses precision on conversion to f64. This is by design.
    ok! { 3.0f32 => "> = 3.0" }
    ok! { 3.1f32 => "> = 3.0999999046325684" }
    ok! { -1.5f32 => "> = -1.5" }
    ok! { 0.5f32 => "> = 0.5" }
    ok! { 0.5e-3f32 => "> = 0.0005000000237487257" }
    ok! { 0.5e+3f32 => "> = 500.0" }
    ok! { 0f32 => "> = 0.0" }
    ok! { f32::MIN => "> = -3.4028234663852886e38" }
    ok! { f32::MAX => "> = 3.4028234663852886e38" }
    ok! { f32::EPSILON => "> = 1.1920928955078125e-7" }

    ser_err! { f32::INFINITY => Error::InfAndNanNotSupported }
    ser_err! { f32::NEG_INFINITY => Error::InfAndNanNotSupported }
    ser_err! { f32::NAN => Error::InfAndNanNotSupported }
}

#[test]
fn f64_val() {
    ok! { 3.0f64 => "> = 3.0" }
    ok! { 3.1f64 => "> = 3.1" }
    ok! { -1.5f64 => "> = -1.5" }
    ok! { 0.5f64 => "> = 0.5" }
    ok! { 0.5e-3f64 => "> = 0.0005" }
    ok! { 0.5e+3f64 => "> = 500.0" }
    ok! { 0f64 => "> = 0.0" }
    ok! { f64::MIN => "> = -1.7976931348623157e308" }
    ok! { f64::MAX => "> = 1.7976931348623157e308" }
    ok! { f64::EPSILON => "> = 2.220446049250313e-16" }

    ser_err! { f64::INFINITY => Error::InfAndNanNotSupported }
    ser_err! { f64::NEG_INFINITY => Error::InfAndNanNotSupported }
    ser_err! { f64::NAN => Error::InfAndNanNotSupported }
}

#[test]
fn char_val() {
    ok! { 'n' => "> = \"n\"" }
    ok! { '"' => "> = \"\\\"\"" }
    ok! { '\\' => "> = \"\\\\\"" }
    ok! { '/' => "> = \"/\"" }
    ok! { '\x08' => "> = \"\\b\"" }
    ok! { '\x0C' => "> = \"\\f\"" }
    ok! { '\n' => "> = \"\\n\"" }
    ok! { '\r' => "> = \"\\r\"" }
    ok! { '\t' => "> = \"\\t\"" }
    ok! { '\x0B' => "> = \"\\u00000b\"" }
    ok! { '\u{3A3}' => "> = \"\u{3A3}\"" }
    ok! { '\u{0}' => "> = \"\\u000000\"" }
}

#[test]
fn str_val() {
    ok! { "".to_string() => r#"> = """# }
    ok! { "foo".to_string() => r#"> = "foo""# }
    ok! { "foo\nbar\u{1}b".to_string() => r#"> = "foo\nbar\u000001b""# }
}

#[test]
fn none_val() {
    ok! { None::<u8> => "> = null" }
}

#[test]
fn some_val() {
    ok! { Some(42u64) => "> = 42" }
}

#[test]
fn unit_val() {
    ok! { () => "> = null" }
}

#[test]
fn unit_struct_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct FooBar;

    ok! { FooBar => "> = null" }
}

#[test]
fn unit_variant_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum UnitValEnum {
        UnitVariant,
    }

    ok! { UnitValEnum::UnitVariant => "> = `UnitVariant`" }
}

#[test]
fn newtype_struct_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct NewtypeStruct(Vec<u8>);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WithNestedNewtypeStruct(NewtypeStruct);

    ok! { NewtypeStruct(vec![1,2,3]) => "> = [1, 2, 3]" };

    ok! { WithNestedNewtypeStruct(NewtypeStruct(vec![1,2,3])) => "> = [1, 2, 3]" };
}

#[test]
fn newtype_variant_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct NewtypeStruct(Vec<u8>);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTypeEnum1 {
        Enum1Variant(NewtypeStruct),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTypeEnum2 {
        Enum2Variant(NewTypeEnum1),
    }

    ok! {
        NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3])) => "> `Enum1Variant` = [1, 2, 3]"
    }

    ok! {
        NewTypeEnum2::Enum2Variant(NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3]))) =>
        "> `Enum2Variant` > `Enum1Variant` = [1, 2, 3]"
    }
}

#[test]
fn seq_val() {
    ok! { vec![true] => "> = [true]" }

    ok! {
        vec![true, false] => indoc! {"
            > = [true, false]\
        "}
    }

    ok! {
        vec![1u8, 2, 3, 4, 5] => indoc! {"
            > = [1, 2, 3, 4, 5]\
        "}
    }

    ok! {
        vec![vec![], vec![], vec![]] as Vec<Vec<()>> => indoc! {"
            > [0] = []

            > [1] = []
            
            > [2] = []\
        "}
    }

    ok! {
        vec![vec![1u8, 2, 3], vec![], vec![]] => indoc! {"
            > [0] = [1, 2, 3]

            > [1] = []

            > [2] = []\
        "}
    }

    ok! {
        vec![vec![], vec![1u8, 2, 3], vec![]] => indoc! {"
            > [0] = []

            > [1] = [1, 2, 3]

            > [2] = []\
        "}
    }

    ok! {
        vec![vec![], vec![], vec![1u8, 2, 3]] => indoc! {"
            > [0] = []

            > [1] = []

            > [2] = [1, 2, 3]\
        "}
    }

    ok! {
        vec![
            vec!["false".to_string()],
            vec![],
            vec!["foo\nbar".to_string(), "3.5".to_string()]
        ] => indoc! {"
            > [0] = [\"false\"]

            > [1] = []

            > [2] = [\"foo\\nbar\", \"3.5\"]\
        "}
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Primitive(usize),
        Compound(Vec<usize>),
    }

    ok! {
        vec![Variant::Primitive(1), Variant::Compound(vec![2])] => indoc! {"
            > [0] > `Primitive` = 1

            > [1] > `Compound` = [2]\
        "}
    }

    ok! { vec![-3, -4, -5] => "> = [-3, -4, -5]" }
    ok! { vec![(), (), ()] => "> = [null, null, null]" }
    ok! { vec![true, false, true] => "> = [true, false, true]" }
    ok! { vec![3.5, 4.5, 5.5] => "> = [3.5, 4.5, 5.5]" }
}

#[test]
fn bytes_val() {
    ok! {
        CString::new([0x01, 0x20, 0x3f]).unwrap() => "> = [1, 32, 63]"
    }
}

#[test]
fn tuple_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Unit,
        String(String),
        Vec(Vec<usize>),
    }

    ok! {
        (5u64, (), 7u64, (6u64, "abc".to_string()), true, vec![42u8]) => indoc! {"
            > [0] = 5

            > [1] = null

            > [2] = 7

            > [3] = [6, \"abc\"]

            > [4] = true

            > [5] = [42]\
        "}
    }

    ok! {
        (5u8,) => "> = [5]"
    }

    ok! {
        (5u64, true, Variant::Unit) => "> = [5, true, `Unit`]"
    }

    ok! {
        (
            Variant::Unit,
            Variant::String("test".into()),
            Variant::Vec(vec![1, 2, 3])
        ) => indoc! {"
            > [0] = `Unit`

            > [1] > `String` = \"test\"

            > [2] > `Vec` = [1, 2, 3]\
        "}
    }
}

#[test]
fn tuple_struct_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Unit,
        String(String),
        Vec(Vec<usize>),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple1(usize, (), u8, (usize, String), bool, Vec<u8>);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple2(u16, u32);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple3(u32, bool, Variant);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple4(Variant, Variant, Variant);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple5(String, Tuple2, Tuple3);

    ok! {
        Tuple1(5, (), 7, (6, "abc".to_string()), true, vec![42u8]) => indoc! {"
            > [0] = 5

            > [1] = null

            > [2] = 7

            > [3] = [6, \"abc\"]

            > [4] = true

            > [5] = [42]\
        "}
    }

    ok! {
        Tuple2(5, 6) => "> = [5, 6]"
    }

    ok! {
        Tuple3(5, true, Variant::Unit) => "> = [5, true, `Unit`]"
    }

    ok! {
        Tuple4(
            Variant::Unit,
            Variant::String("test".into()),
            Variant::Vec(vec![1, 2, 3])
        ) => indoc! {"
            > [0] = `Unit`

            > [1] > `String` = \"test\"

            > [2] > `Vec` = [1, 2, 3]\
        "}
    }

    ok! {
        Tuple5(
            "Hello".into(),
            Tuple2(42, 24),
            Tuple3(43, false, Variant::Vec(vec![44, 45]))
        ) => indoc! {"
            > [0] = \"Hello\"

            > [1] = [42, 24]

            > [2] > [0] = 43

            > [2] > [1] = false

            > [2] > [2] > `Vec` = [44, 45]\
        "}
    }
}

#[test]
fn tuple_variant_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Tuple1(usize, (), u8, (usize, String), bool, Vec<u8>),
        Tuple2(u16, u32),
    }

    ok! {
        vec![
            Variant::Tuple1(5, (), 7, (6, "abc".to_string()), true, vec![42u8]),
            Variant::Tuple2(42, 43)
        ] => indoc! {"
            > [0] > `Tuple1` > [0] = 5

            > [0] > `Tuple1` > [1] = null

            > [0] > `Tuple1` > [2] = 7

            > [0] > `Tuple1` > [3] = [6, \"abc\"]

            > [0] > `Tuple1` > [4] = true

            > [0] > `Tuple1` > [5] = [42]

            > [1] > `Tuple2` = [42, 43]\
        "}
    }
}

#[test]
fn map_val() {
    #[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    enum Enum {
        UnitVariant,
        NewTypeVariant(String),
        TupleVariant(u8, u8),
        StructVariant { foo: u8 },
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    struct NewtypeStruct(String);

    #[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    struct UnitStruct;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    struct TupleStruct(u8, u8);

    #[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
    struct Struct {
        foo: u8,
    }

    ok! {
        map!["a".to_string() => true] => "> [\"a\"] = true"
    }

    ok! {
        map!["a".to_string() => true, "b".to_string() => false] => indoc! {"
            > [\"a\"] = true

            > [\"b\"] = false\
        "}
    }

    ok! {
        map![
            "a".to_string() => vec![],
            "b".to_string() => vec![],
            "c".to_string() => vec![]
        ] as BTreeMap<_, Vec<()>> => indoc! {"
            > [\"a\"] = []

            > [\"b\"] = []
            
            > [\"c\"] = []\
        "}
    }

    ok! {
        map![
            "a".to_string() => map![
                "a".to_string() => vec![1u64, 2u64, 3u64],
                "b".to_string() => vec![]
            ],
            "b".to_string() => map!["a".to_string() => vec![]],
            "c".to_string() => map!["a".to_string() => vec![]]
        ] => indoc! {"
            > [\"a\"] > [\"a\"] = [1, 2, 3]

            > [\"a\"] > [\"b\"] = []
            
            > [\"b\"] > [\"a\"] = []
            
            > [\"c\"] > [\"a\"] = []\
        "}
    }

    ok! {
        map![
            "a".to_string() => map!["a".to_string() => vec![]],
            "b".to_string() => map![
                "a".to_string() => vec![1u64, 2u64, 3u64],
                "b".to_string() => vec![]
            ],
            "c".to_string() => map!["a".to_string() => vec![]]
        ] => indoc! {"
            > [\"a\"] > [\"a\"] = []

            > [\"b\"] > [\"a\"] = [1, 2, 3]
            
            > [\"b\"] > [\"b\"] = []
            
            > [\"c\"] > [\"a\"] = []\
        "}
    }

    ok! {
        map![
            "abc xyz".to_string() => map![
                "42".to_string() => vec![]
            ],
            "bc xyz".to_string() => map![
                "Hello world!".to_string() => vec![]
            ],
            "c xyz".to_string() => map![
                "a".to_string() => vec![1u64, 2u64, 3u64],
                "b".to_string() => vec![]
            ]
        ] => indoc! {"
            > [\"abc xyz\"] > [\"42\"] = []

            > [\"bc xyz\"] > [\"Hello world!\"] = []
            
            > [\"c xyz\"] > [\"a\"] = [1, 2, 3]
            
            > [\"c xyz\"] > [\"b\"] = []\
        "}
    }

    ok! {
        map!["c".to_string() => ()] => "> [\"c\"] = null"
    }

    ok! {
        map!["c\nb".to_string() => ()] => "> [\"c\\nb\"] = null"
    }

    ok! {
        map!["b".to_string() => vec![
            map!["c".to_string() => "\x0c\x1f\r".to_string()],
            map!["d".to_string() => "".to_string()]
        ]] => indoc!{"
            > [\"b\"] > [0] > [\"c\"] = \"\\f\\u00001f\\r\"

            > [\"b\"] > [1] > [\"d\"] = \"\"\
        "}
    }

    ok! {
        map![0u8 => (), 1u8 => (), 2u8 => ()] => indoc!{"
            > [\"0\"] = null

            > [\"1\"] = null

            > [\"2\"] = null\
        "}
    }

    ok! { map![42u8 => 43u64] => "> [\"42\"] = 43" }
    ok! { map![42u16 => 43u64] => "> [\"42\"] = 43" }
    ok! { map![42u32 => 43u64] => "> [\"42\"] = 43" }
    ok! { map![42u64 => 43u64] => "> [\"42\"] = 43" }

    ok! { map![-42i8 => 43u64] => "> [\"-42\"] = 43" }
    ok! { map![-42i16 => 43u64] => "> [\"-42\"] = 43" }
    ok! { map![-42i32 => 43u64] => "> [\"-42\"] = 43" }
    ok! { map![-42i64 => 43u64] => "> [\"-42\"] = 43" }

    ok! { map!['a' => 43u64] => "> [\"a\"] = 43"}
    ok! { map!['\n' => 43u64] => "> [\"\\n\"] = 43" }
    ok! { map!['\u{0}' => 43u64] => "> [\"\\u000000\"] = 43" }

    ok! { map!["\n".to_string() => 43u64] => "> [\"\\n\"] = 43" }
    ok! { map!["\u{0}".to_string() => 43u64] => "> [\"\\u000000\"] = 43" }

    ok! { map![true => 43u64] => "> [\"true\"] = 43" }
    ok! { map![Some(42u8) => 43u64] => "> [\"42\"] = 43" }

    ok! { map![Enum::UnitVariant => 43u64] => "> [\"UnitVariant\"] = 43" }
    ok! { map![NewtypeStruct("foo".into()) => 43u64] => "> [\"foo\"] = 43" }

    ser_err! { map![UnitStruct => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![Enum::NewTypeVariant("foo".into()) => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![vec![1u8, 2u8] => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![CString::new([0x01]).unwrap() => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![TupleStruct(1, 2) => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![Enum::TupleVariant(1, 2) => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![map![1 => 2] => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![Struct { foo: 1 } => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![Enum::StructVariant { foo: 1 } => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![(1u8, 2u8) => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![() => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![Option::<u8>::None => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![42u128 => 43u64] => Error::InvalidMapKeyType }
    ser_err! { map![-42i128 => 43u64] => Error::InvalidMapKeyType }
}

#[test]
fn struct_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Struct {
        null: (),
        boolean: bool,
        different_ints: Ints,
        some_floats: Floats,
        string: NewtypeString,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Ints {
        pos: u64,
        neg: i64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Floats {
        first: f64,
        second: f64,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct NewtypeString(String);

    ok! {
        Struct {
            null: (),
            boolean: true,
            different_ints: Ints {
                pos: 42,
                neg: -1337,
            },
            some_floats: Floats {
                first: 3.14,
                second: 42.42,
            },
            string: NewtypeString("hello".into()),
        } => indoc!{"
            > null = null

            > boolean = true

            > different_ints > pos = 42

            > different_ints > neg = -1337

            > some_floats > first = 3.14

            > some_floats > second = 42.42

            > string = \"hello\"\
        "}
    }
}

#[test]
fn struct_variant_val() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Struct1 {
            usize_field: usize,
            vec_field: Vec<usize>,
            tuple_field: (usize, String, Vec<usize>),
        },
        Struct2 {
            str_field: String,
        },
    }

    ok! {
        vec![
            Variant::Struct1 {
                usize_field: 42,
                vec_field: vec![43, 44],
                tuple_field: (45, "Hello".into(), vec![46, 47, 48])
            },
            Variant::Struct2 {
                str_field: "World".into()
            }

        ] => indoc! {"
            > [0] > `Struct1` > usize_field = 42

            > [0] > `Struct1` > vec_field = [43, 44]

            > [0] > `Struct1` > tuple_field > [0] = 45

            > [0] > `Struct1` > tuple_field > [1] = \"Hello\"

            > [0] > `Struct1` > tuple_field > [2] = [46, 47, 48]

            > [1] > `Struct2` > str_field = \"World\"\
        "}
    }
}
