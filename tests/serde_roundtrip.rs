use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

macro_rules! ok {
    ($rust:expr => $kfg:expr) => {
        let serialized = konfig::to_string(&$rust).unwrap();

        assert_eq!(serialized, $kfg);
    };
}

macro_rules! tm {
    () => {
        BTreeMap::new()
    };

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
    ok! { () => "> = none" }
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
#[ignore]
fn object_value() {
    ok! {
        tm!["a" => true] => "> a = true"
    }

    ok! {
        tm!["a" => true, "b" => false] => indoc! {"
            > a = true

            > b = false
        "}
    }

    ok! {
        tm!["a" => vec![], "b" => vec![], "c" => vec![]] as BTreeMap<_, Vec<()>> => indoc! {"
            > a = []

            > b = []
            
            > c = []
        "}
    }

    ok! {
        tm![
            "a" => tm!["a" => vec![1, 2, 3], "b" => vec![]],
            "b" => tm!["a" => vec![]],
            "c" => tm!["a" => vec![]]
        ] => indoc! {"
            > a > a = [1, 2, 3]

            > a > b = []
            
            > b > a = []
            
            > c > a = []
        "}
    }

    ok! {
        tm![
            "a" => tm!["a" => vec![]],
            "b" => tm!["a" => vec![1, 2, 3], "b" => vec![]],
            "c" => tm!["a" => vec![]]
        ] => indoc! {"
            > a > a = []

            > b > a = [1, 2, 3]
            
            > b > b = []
            
            > c > a = []
        "}
    }

    ok! {
        tm![
            "a" => tm!["a" => vec![]],
            "b" => tm!["a" => vec![]],
            "c" => tm!["a" => vec![1, 2, 3], "b" => vec![]]
        ] => indoc! {"
            > a > a = []

            > b > b = []
            
            > c > a = [1, 2, 3]
            
            > c > b = []
        "}
    }

    ok! {
        tm!["c" => ()] => "> c = none"
    }

    ok! {
        tm!["b" => vec![
            tm!["c" => "\x0c\x1f\r"],
            tm!["d" => ""]
        ]] => indoc!{r#"
            > b > c = "\f\u001f\r"

            > b > d = "" 
        "#}
    }
}

#[test]
#[ignore]
fn tuple_value() {
    ok! { (5,) => "> [0] = 5" };

    ok! {
       (5, (6, "abc")) => indoc!{"
            > [0] = 5

            > [1] > [0] = 6

            > [1] > [1] = \"abc\"
        "}
    };
}

#[test]
#[ignore]
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

    ok! {
        Animal::Frog("Henry".to_string(), vec![]) => indoc!{"
            > = `Frog`

            > Frog > [0] = \"Henry\"
            
            > Frog > [1] = []
        "}
    };

    ok! {
        Animal::Frog("Henry".to_string(), vec![349]) => indoc!{"
            > = `Frog`

            > Frog > [0] = \"Henry\"
            
            > Frog > [1] = [349]
        "}
    };

    ok! {
        Animal::Frog("Henry".to_string(), vec![349, 102]) => indoc!{"
            > = `Frog`

            > Frog > [0] = \"Henry\"
            
            > Frog > [1] = [349, 102]
        "}
    };

    ok! {
        Animal::Cat {
            age: 5,
            name: "Kate".to_string(),
        } => indoc!{"
            > = `Cat`

            > Cat > age = 5
            
            > Cat > name = \"Kate\"
        "}
    };

    ok! {
        Animal::AntHive(vec!["Bob".to_string(), "Stuart".to_string()]) => indoc!{"
            > = `AntHive`
            
            > AntHive = [\"Bob\", \"Stuart\"]
        "}
    };
}
