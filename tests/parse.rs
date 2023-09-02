use indoc::indoc;
use konfig::parser::parse;
use konfig::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// NOTE: this pretty much a direct copy of the `crate::Value`, but without custom
// Serialize/Deserialize implementation allowing us to test resulting AST comparing
// it with its RON representation. Not very elegant, but convenient.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AstValue {
    Sequence(Vec<AstValue>),
    Map(HashMap<String, AstValue>),
    Struct(HashMap<String, AstValue>),
    Variant(String, Box<AstValue>),
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
    UnitVariant(String),
}

impl<T> From<T> for AstValue
where
    T: Into<Value>,
{
    fn from(value: T) -> Self {
        match value.into() {
            Value::Sequence(s) => AstValue::Sequence(s.into_iter().map(Into::into).collect()),
            Value::Map(m) => AstValue::Map(m.into_iter().map(|(k, v)| (k, v.into())).collect()),
            Value::Struct(s) => {
                AstValue::Struct(s.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            Value::Variant(name, val) => AstValue::Variant(name, Box::new(val.into())),
            Value::Null => AstValue::Null,
            Value::Bool(v) => AstValue::Bool(v),
            Value::PosInt(v) => AstValue::PosInt(v),
            Value::NegInt(v) => AstValue::NegInt(v),
            Value::Float(v) => AstValue::Float(v),
            Value::String(v) => AstValue::String(v),
            Value::UnitVariant(v) => AstValue::UnitVariant(v),
        }
    }
}

macro_rules! ok {
    ($input:expr => $($expected:tt)+) => {{
        let parsed = match parse($input) {
            Ok(parsed) => parsed,
            Err(err) => panic!("\n{}", err)
        };

        let actual = AstValue::from(parsed);
        let expected = ron::from_str::<AstValue>(stringify!( $($expected)+)).unwrap();

        assert_eq!(actual, expected);
    }};
}

macro_rules! err {
    ($input:expr => $expected:expr) => {
        let actual = parse($input).unwrap_err().to_string();
        let expected = indoc!($expected);

        assert_eq!(
            actual, expected,
            "expected:\n\n{expected}\n\nbut got:\n\n{actual}"
        );
    };
}

#[test]
fn simple_assignment() {
    ok! {
        "> foo_bar > [0] > `Baz` > ['qux quz'] = [1, 2, 3]" =>
        Struct({
            "foo_bar": Sequence([
                Variant(
                    "Baz",
                    Map({
                        "qux quz" : Sequence([
                            PosInt(1),
                            PosInt(2),
                            PosInt(3)
                        ])
                    })
                )
            ])
        })
    }

    ok! {
        "> = `Hello`" => UnitVariant("Hello")
    }

    ok! {
        "> foo_bar = `Hello`" =>
        Struct({
            "foo_bar": UnitVariant("Hello")
        })
    }

    ok! {
        "> ['>'] = `Hello`" =>
        Map({
            ">": UnitVariant("Hello")
        })
    }
}

#[test]
fn assignment_spacing() {
    ok! {
        "> `Hello` >  \n>  `World` = true" =>
        Variant(
            "Hello",
            Variant(
                "World",
                Bool(true)
            )
        )
    }

    ok! {
        ">   foo   >bar> \n>baz = true" =>
        Struct({
            "foo": Struct({
                "bar": Struct({
                    "baz": Bool(true)
                })
            })
        })
    }

    ok! {
        ">foo>\n>bar>\n>baz=true" =>
        Struct({
            "foo": Struct({
                "bar": Struct({
                    "baz": Bool(true)
                })
            })
        })
    }

    ok! {
        ">foo=42" =>
        Struct({
            "foo": PosInt(42)
        })
    }

    ok! {
        "> foo      =      42" =>
        Struct({
            "foo": PosInt(42)
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43
        " =>
        Struct({
            "foo": PosInt(42),
            "bar": PosInt(43)
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43" =>
        Struct({
            "foo": PosInt(42),
            "bar": PosInt(43)
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43  " =>
        Struct({
            "foo": PosInt(42),
            "bar": PosInt(43)
        })
    }

    ok! {
        "

    
            > foo = 42

            > bar = 43


        " =>
        Struct({
            "foo": PosInt(42),
            "bar": PosInt(43)
        })
    }

    ok! {
        "> foo  \n =  42" =>
        Struct({
            "foo": PosInt(42)
        })
    }

    err! {
        "   > foo   \n   =   \n   42" =>
        " --> 2:8
        |
      2 |    =   ␊
        |        ^---
        |
        = expected assignment right hand side"
    }

    err! {
        indoc! {"
            > foo = 42
            > bar = 43
        "} =>
        " --> 1:11
        |
      1 | > foo = 42␊
        |           ^---
        |
        = expected double new line or end of input"
    }

    err! {
        "> foo = 42 > bar = 43" =>
        " --> 1:11
        |
      1 | > foo = 42 > bar = 43
        |           ^---
        |
        = expected double new line or end of input"
    }

    err! {
        "> ['foo'\n] = 42" =>
        " --> 1:8
        |
      1 | > ['foo'
        |        ^---
        |
        = expected single quoted string or escape sequence"
    }

    err! {
        "> [\n'foo'] = 42" =>
        " --> 1:4
        |
      1 | > [␊
        |    ^---
        |
        = expected map key or sequence index"
    }

    err! {
        "> [0\n] = 42" =>
        " --> 1:4
        |
      1 | > [0
        |    ^---
        |
        = expected map key"
    }

    err! {
        "> [\n0] = 42" =>
        " --> 1:4
        |
      1 | > [␊
        |    ^---
        |
        = expected map key or sequence index"
    }

    err! {
        "> foo >\n\n> bar = 42" =>
        " --> 1:8
        |
      1 | > foo >␊
        |        ^---
        |
        = expected path item"
    }

    err! {
        "> \n > foo = 42" =>
        " --> 1:3
        |
      1 | > ␊
        |   ^---
        |
        = expected path item"
    }

    err! {
        "> foo \n \n = 42" =>
        " --> 1:1
        |
      1 | > foo 
        | ^---
        |
        = expected expression"
    }

    err! {
        "> foo = \n \n 42" =>
        " --> 1:9
        |
      1 | > foo = ␊
        |         ^---
        |
        = expected assignment right hand side"
    }

    err! { "> > foo_bar = 123" =>
        " --> 1:3
        |
      1 | > > foo_bar = 123
        |   ^---
        |
        = expected path item"
    }
}

#[test]
fn sequence_of_primitives_spacing() {
    ok! {
        "> = [ \n  1.3e+10,  'foo',true \n  , 42  \n, \n ]" =>
        Sequence([
            Float(1.3e+10),
            String("foo"),
            Bool(true),
            PosInt(42),
        ])
    }

    ok! {
        "> = [ 1\n  ]" =>
        Sequence([
            PosInt(1),
        ])
    }

    ok! {
        "> = [\n1]" =>
        Sequence([
            PosInt(1),
        ])
    }

    ok! {
        "> = [1,\n]" =>
        Sequence([
            PosInt(1),
        ])
    }

    err! {
        "> = [\n1\n\n]" =>
        " --> 2:1
        |
      2 | 1
        | ^---
        |
        = expected `null`, boolean value, negative integer, or floating point number"
    }

    err! {
        "> = [1,\n\n]" =>
        " --> 2:1
        |
      2 | ␊
        | ^---
        |
        = expected primitive value"
    }

    err! {
        "> = [1,\n,]" =>
        " --> 2:1
        |
      2 | ,]
        | ^---
        |
        = expected primitive value"
    }
}

#[test]
fn empty_sequence_of_primitives_spacing() {
    ok! {
        "> = []" =>
        Sequence([])
    }

    ok! {
        "> = [             ]" =>
        Sequence([])
    }

    ok! {
        "> = [
        ]" =>
        Sequence([])
    }

    err! {
        "> = [

        ]" =>
        " --> 2:1
        |
      2 | ␊
        | ^---
        |
        = expected primitive value"
    }
}

#[test]
fn numbers_spacing() {
    err! {
        "> = - 3" =>
        " --> 1:6
        |
      1 | > = - 3
        |      ^---
        |
        = expected digits"
    }

    err! {
        "> = 3. 45" =>
        " --> 1:7
        |
      1 | > = 3. 45
        |       ^---
        |
        = expected double new line or end of input"
    }

    err! {
        "> = 3.45 e+10" =>
        " --> 1:9
        |
      1 | > = 3.45 e+10
        |         ^---
        |
        = expected double new line or end of input"
    }

    err! {
        "> = 3.45e +10" =>
        " --> 1:9
        |
      1 | > = 3.45e +10
        |         ^---
        |
        = expected double new line or end of input"
    }

    err! {
        "> = 3.45e+ 10" =>
        " --> 1:9
        |
      1 | > = 3.45e+ 10
        |         ^---
        |
        = expected double new line or end of input"
    }
}

#[test]
fn parse_path_item() {
    ok! { "> foo_bar = null" => Struct({ "foo_bar": Null }) }
    ok! { "> `FooBar` = null" => Variant("FooBar", Null) }
    ok! { "> [0] = null" => Sequence([Null]) }
    ok! { "> [\"foobar\"] = null" => Map({ "foobar": Null }) }
    ok! { "> ['foobar'] = null" => Map({ "foobar": Null }) }
}

#[test]
fn parse_rhs() {
    ok! {
        indoc! {r#"
            > enum_variant = `Foo`

            > seq_of_primitives = [
                "foo", 'bar',
                1, 2.3e1,
                null
            ]

            > null = null

            > bool > true = true

            > bool > false = false

            > pos_int > dec = 42
            
            > pos_int > hex > hi = 0x2A

            > pos_int > hex > lo = 0x2a

            > float > [0] = 42.

            > float > [1] = 42.42

            > float > [2] = 1.956e-10

            > string > raw =
            ```test
             foo bar 
            ```

            > string > double = " foo bar "

            > string > single = ' foo bar '

        "#} => Struct({
            "enum_variant": UnitVariant("Foo"),
            "seq_of_primitives": Sequence([
                String("foo"),
                String("bar"),
                PosInt(1),
                Float(2.3e1),
                Null
            ]),
            "null": Null,
            "bool": Struct({
                "true": Bool(true),
                "false": Bool(false)
            }),
            "pos_int": Struct({
                "dec": PosInt(42),
                "hex": Struct({
                    "hi": PosInt(42),
                    "lo": PosInt(42)
                })
            }),
            "float": Sequence([
                Float(42.0),
                Float(42.42),
                Float(1.956e-10),
            ]),
            "string": Struct({
                "raw": String(" foo bar "),
                "double": String(" foo bar "),
                "single": String(" foo bar ")
            })
        })
    }
}

#[test]
fn sequence_overwrite() {
    err! {
        indoc! {"
            > foo = [1, 2, 3]

            > foo > [0] = 3
        "} => 
        " --> 3:9
        |
      3 | > foo > [0] = 3
        |         ^-^
        |
        = path item is expected to be sequence, but it was previously defined as inline sequence of primitive values"
    }

    err! {
        indoc! {"
            > foo > [0] = 3

            > foo = [1, 2, 3]
        "} => 
        " --> 3:1
        |
      3 | > foo = [1, 2, 3]␊
        | ^----------------^
        |
        = attempt to assign inline sequence of primitive values to a path item that was previously defined as sequence"
    }
}

#[test]
fn doc_and_expr_spacing() {
    ok! {
        indoc! {"
            Hello world!

            > foo = 3      
            
            * Hey!
            > bar = 4

            Lorem ipsum
            123
        "} => 
        Struct({
            "foo": PosInt(3),
            "bar": PosInt(4)
        })
    }

    ok! {
        indoc! {"
            Hello world!
            > foo = 
            ```rust
            foo
            bar

            baz
            ```

            * Hey!
            
            Lorem ipsum
            123
        "} => 
        Struct({
            "foo": String("foo\nbar\n\nbaz"),
        })
    }

    err! {
        indoc! {"
            Hello world!
            
            > foo = 3
            * Hey!
            > bar = 4

            Lorem ipsum
            123
        "} => 
        " --> 3:10
        |
      3 | > foo = 3␊
        |          ^---
        |
        = expected double new line or end of input"
    }

    err! {
        indoc! {"
            Hello world!
            > foo = 
            ```rust
            foo
            bar

            baz
            ```
            * Hey!
        "} => 
        " --> 8:4
        |
      8 | ```␊
        |    ^---
        |
        = expected double new line or end of input"
    }

    err! {
        indoc! {"
            Hello world!
                     > *Hi there
            > foo = 42
        "} => 
        " --> 2:12
        |
      2 |          > *Hi there
        |            ^---
        |
        = expected path item"
    }
}
