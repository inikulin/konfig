use indoc::indoc;
use konfig::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

macro_rules! ok {
    ($rust:expr => $kfg:expr) => {
        let serialized = konfig::to_string(&$rust).unwrap();

        assert_eq!(serialized, $kfg);
    };
}

macro_rules! err {
    ($rust:expr => $err:expr) => {
        let err = konfig::to_string(&$rust).unwrap_err();

        assert_eq!(err, $err);
    };
}

macro_rules! map {
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = BTreeMap::new();
            $(
                m.insert($k.clone(), $v.clone());
            )+
            m
        }
    };
}

#[test]
fn none_val() {
    ok! { () => "> = null" }
    ok! { None::<u8> => "> = null" }
}

#[test]
fn u64_val() {
    ok! { 3u64 => "> = 3" }
    ok! { u64::MAX => format!("> = {}", u64::MAX) }
}

#[test]
fn i64_val() {
    ok! { 3i64 => "> = 3" }
    ok! { -2i64 => "> = -2" }
    ok! { -1234i64 => "> = -1234" }
    ok! { i64::MIN => format!("> = {}", i64::MIN) }
}

#[test]
fn f64_val() {
    ok! { 3.0 => "> = 3.0" }
    ok! { 3.1 => "> = 3.1" }
    ok! { -1.5 => "> = -1.5" }
    ok! { 0.5 => "> = 0.5" }
    ok! { 0.5 => "> = 0.5" }
    ok! { f64::MIN => "> = -1.7976931348623157e308" }
    ok! { f64::MAX => "> = 1.7976931348623157e308" }
    ok! { f64::EPSILON => "> = 2.220446049250313e-16" }
}

#[test]
fn ints128_val() {
    err! { 0u128 => Error::Int128NotSupported }
    err! { 0i128 => Error::Int128NotSupported }
}

#[test]
fn str_val() {
    ok! { "" => r#"> = """# }
    ok! { "foo" => r#"> = "foo""# }
    ok! { "foo\nbar\u{1}b" => r#"> = "foo\nbar\u0001b""# }
}

#[test]
fn bool_val() {
    ok! { true => "> = true" }
    ok! { false => "> = false" }
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
    ok! { '\x0B' => "> = \"\\u000b\"" }
    ok! { '\u{3A3}' => "> = \"\u{3A3}\"" }
}

#[test]
fn bytes_val() {
    ok! {
        std::ffi::CString::new([0x01, 0x20, 0x3f]).unwrap() => "> = [0x01, 0x20, 0x3F]"
    }
}

#[test]
fn list_val() {
    ok! { vec![true] => "> = [true]" }

    ok! {
        vec![true, false] => indoc! {"
            > = [true, false]\
        "}
    }

    ok! {
        vec![1, 2, 3, 4, 5] => indoc! {"
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
        vec![vec![1, 2, 3], vec![], vec![]] => indoc! {"
            > [0] = [1, 2, 3]

            > [1] = []

            > [2] = []\
        "}
    }

    ok! {
        vec![vec![], vec![1, 2, 3], vec![]] => indoc! {"
            > [0] = []

            > [1] = [1, 2, 3]

            > [2] = []\
        "}
    }

    ok! {
        vec![vec![], vec![], vec![1, 2, 3]] => indoc! {"
            > [0] = []

            > [1] = []

            > [2] = [1, 2, 3]\
        "}
    }

    ok! {
        vec![vec!["false"], vec![], vec!["foo\nbar", "3.5"]] => indoc! {"
            > [0] = [\"false\"]

            > [1] = []

            > [2] = [\"foo\\nbar\", \"3.5\"]\
        "}
    }
}

#[test]
fn list_of_enums() {
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
}

#[test]
fn list_of_dynamics() {
    // NOTE: primitive value checker can't see into erased values, so we serialize those as compound
    let list: Vec<Box<dyn erased_serde::Serialize>> = vec![Box::new(42), Box::new(vec![43])];

    ok! {
        list => indoc! {"
            > [0] = 42

            > [1] > [0] = 43\
        "}
    }

    let list: Vec<Box<dyn erased_serde::Serialize>> = vec![Box::new(42), Box::new(43)];

    ok! {
        list => indoc! {"
            > [0] = 42

            > [1] = 43\
        "}
    }
}

#[test]
fn tuple_value() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Unit,
        String(String),
        Vec(Vec<usize>),
    }

    ok! {
        (5, (), 7, (6, "abc"), true, vec![42u8]) => indoc! {"
            > [0] = 5

            > [1] = null

            > [2] = 7

            > [3] = [6, \"abc\"]

            > [4] = true

            > [5] = [42]\
        "}
    }

    ok! {
        (5,) => "> = [5]"
    }

    ok! {
        (5, true, Variant::Unit) => "> = [5, true, `Unit`]"
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
fn tuple_struct_value() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Unit,
        String(String),
        Vec(Vec<usize>),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple1(usize, (), u8, (usize, String), bool, Vec<u8>);

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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple2(u16, u32);

    ok! {
        Tuple2(5, 6) => "> = [5, 6]"
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple3(i32, bool, Variant);

    ok! {
        Tuple3(5, true, Variant::Unit) => "> = [5, true, `Unit`]"
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple4(Variant, Variant, Variant);

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

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    struct Tuple5(String, Tuple2, Tuple3);

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
fn tuple_enum_variant_value() {
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
fn struct_enum_variant() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    enum Variant {
        Struct1 {
            usize_field: usize,
            vec_field: Vec<usize>,
            tuple_field: (usize, String, Vec<usize>),
        },
        Struct2 {
            str_field: &'static str,
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
                str_field: "World"
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

#[test]
fn map_value() {
    ok! {
        map!["a" => true] => "> [\"a\"] = true"
    }

    ok! {
        map!["a" => true, "b" => false] => indoc! {"
            > [\"a\"] = true

            > [\"b\"] = false\
        "}
    }

    ok! {
        map!["a" => vec![], "b" => vec![], "c" => vec![]] as BTreeMap<_, Vec<()>> => indoc! {"
            > [\"a\"] = []

            > [\"b\"] = []
            
            > [\"c\"] = []\
        "}
    }

    ok! {
        map![
            "a" => map!["a" => vec![1, 2, 3], "b" => vec![]],
            "b" => map!["a" => vec![]],
            "c" => map!["a" => vec![]]
        ] => indoc! {"
            > [\"a\"] > [\"a\"] = [1, 2, 3]

            > [\"a\"] > [\"b\"] = []
            
            > [\"b\"] > [\"a\"] = []
            
            > [\"c\"] > [\"a\"] = []\
        "}
    }

    ok! {
        map![
            "a" => map!["a" => vec![]],
            "b" => map!["a" => vec![1, 2, 3], "b" => vec![]],
            "c" => map!["a" => vec![]]
        ] => indoc! {"
            > [\"a\"] > [\"a\"] = []

            > [\"b\"] > [\"a\"] = [1, 2, 3]
            
            > [\"b\"] > [\"b\"] = []
            
            > [\"c\"] > [\"a\"] = []\
        "}
    }

    ok! {
        map![
            "abc xyz" => map!["42" => vec![]],
            "bc xyz" => map!["Hello world!" => vec![]],
            "c xyz" => map!["a" => vec![1, 2, 3], "b" => vec![]]
        ] => indoc! {"
            > [\"abc xyz\"] > [\"42\"] = []

            > [\"bc xyz\"] > [\"Hello world!\"] = []
            
            > [\"c xyz\"] > [\"a\"] = [1, 2, 3]
            
            > [\"c xyz\"] > [\"b\"] = []\
        "}
    }

    ok! {
        map!["c" => ()] => "> [\"c\"] = null"
    }

    ok! {
        map!["c\nb" => ()] => "> [\"c\\nb\"] = null"
    }

    ok! {
        map!["b" => vec![
            map!["c" => "\x0c\x1f\r"],
            map!["d" => ""]
        ]] => indoc!{"
            > [\"b\"] > [0] > [\"c\"] = \"\\f\\u001f\\r\"

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
}

#[test]
fn enum_value() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(deny_unknown_fields)]
    enum Animal {
        Dog,
        Frog(String, Vec<isize>),
        Cat { age: usize, name: String },
        AntHive(Vec<String>),
    }

    ok! { Animal::Dog => "> = `Dog`" };

    ok!(
        Animal::Frog("Henry".to_string(), vec![]) => indoc!{"
            > `Frog` > [0] = \"Henry\"
        
            > `Frog` > [1] = []\
        "}
    );

    ok! {
        Animal::Cat {
            age: 5,
            name: "Kate".to_string(),
        } => indoc!{"
            > `Cat` > age = 5

            > `Cat` > name = \"Kate\"\
        "}
    };

    ok! {
        Animal::AntHive(vec!["Bob".to_string(), "Stuart".to_string()]) =>
        "> `AntHive` = [\"Bob\", \"Stuart\"]"
    };
}

#[test]
fn newtype_struct_and_variant() {
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct NewtypeStruct(Vec<u8>);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct WithNestedNewtypeStruct(NewtypeStruct);

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTypeEnum1 {
        Enum1Variant(NewtypeStruct),
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NewTypeEnum2 {
        Enum2Variant(NewTypeEnum1),
    }

    ok! { NewtypeStruct(vec![1,2,3]) => "> = [1, 2, 3]" };

    ok! { WithNestedNewtypeStruct(NewtypeStruct(vec![1,2,3])) => "> = [1, 2, 3]" };

    ok! {
        NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3])) => "> `Enum1Variant` = [1, 2, 3]"
    };

    ok! {
        NewTypeEnum2::Enum2Variant(NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3]))) =>
        "> `Enum2Variant` > `Enum1Variant` = [1, 2, 3]"
    };
}
