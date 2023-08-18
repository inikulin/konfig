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
    ($rust:expr, $err:expr) => {
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
    ok! { () => "" }
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
fn tuple_value() {
    err!((5,), Error::TuplesUnsupported);
    err!((5, (6, "abc")), Error::TuplesUnsupported);
}

#[test]
fn map_value() {
    ok! {
        map!["a" => true] => "> [a] = true"
    }

    ok! {
        map!["a" => true, "b" => false] => indoc! {"
            > [a] = true

            > [b] = false\
        "}
    }

    ok! {
        map!["a" => vec![], "b" => vec![], "c" => vec![]] as BTreeMap<_, Vec<()>> => indoc! {"
            > [a] = []

            > [b] = []
            
            > [c] = []\
        "}
    }

    ok! {
        map![
            "a" => map!["a" => vec![1, 2, 3], "b" => vec![]],
            "b" => map!["a" => vec![]],
            "c" => map!["a" => vec![]]
        ] => indoc! {"
            > [a] > [a] = [1, 2, 3]

            > [a] > [b] = []
            
            > [b] > [a] = []
            
            > [c] > [a] = []\
        "}
    }

    ok! {
        map![
            "a" => map!["a" => vec![]],
            "b" => map!["a" => vec![1, 2, 3], "b" => vec![]],
            "c" => map!["a" => vec![]]
        ] => indoc! {"
            > [a] > [a] = []

            > [b] > [a] = [1, 2, 3]
            
            > [b] > [b] = []
            
            > [c] > [a] = []\
        "}
    }

    ok! {
        map![
            "abc xyz" => map!["42" => vec![]],
            "bc xyz" => map!["Hello world!" => vec![]],
            "c xyz" => map!["a" => vec![1, 2, 3], "b" => vec![]]
        ] => indoc! {"
            > [abc xyz] > [42] = []

            > [bc xyz] > [Hello world!] = []
            
            > [c xyz] > [a] = [1, 2, 3]
            
            > [c xyz] > [b] = []\
        "}
    }

    ok! {
        map!["c" => ()] => ""
    }

    ok! {
        map!["b" => vec![
            map!["c" => "\x0c\x1f\r"],
            map!["d" => ""]
        ]] => indoc!{"
            > [b] > [0] > [c] = \"\\f\\u001f\\r\"

            > [b] > [1] > [d] = \"\"\
        "}
    }

    ok! {
        map![0u8 => (), 1u8 => (), 2u8 => ()] => ""
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

    err!(
        Animal::Frog("Henry".to_string(), vec![]),
        Error::TuplesUnsupported
    );

    err! {
        Animal::Cat {
            age: 5,
            name: "Kate".to_string(),
        },
        Error::StructVariantsUnsupported
    };

    ok! {
        Animal::AntHive(vec!["Bob".to_string(), "Stuart".to_string()]) => indoc!{"
            > = `AntHive`
            
            > `AntHive` = [\"Bob\", \"Stuart\"]\
        "}
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
        NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3])) => indoc!{"
            > = `Enum1Variant`

            > `Enum1Variant` = [1, 2, 3]\
        "}
    };

    ok! {
        NewTypeEnum2::Enum2Variant(NewTypeEnum1::Enum1Variant(NewtypeStruct(vec![1,2,3])))
        => indoc!{"
            > = `Enum2Variant`

            > `Enum2Variant` = `Enum1Variant`

            > `Enum2Variant` > `Enum1Variant` = [1, 2, 3]\
        "}
    };
}
