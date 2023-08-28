use indoc::indoc;
use konfig::parser::parse;
use konfig::value::{Primitive, Value, ValueCell};
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
    Primitive(AstPrimitive),
}

impl From<ValueCell> for AstValue {
    fn from(value: ValueCell) -> Self {
        match Value::from(value) {
            Value::Sequence(s) => AstValue::Sequence(s.into_iter().map(Into::into).collect()),
            Value::Map(m) => AstValue::Map(m.into_iter().map(|(k, v)| (k, v.into())).collect()),
            Value::Struct(s) => {
                AstValue::Struct(s.into_iter().map(|(k, v)| (k, v.into())).collect())
            }
            Value::Variant(name, val) => AstValue::Variant(name, Box::new(val.into())),
            Value::Primitive(p) => AstValue::Primitive(p.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AstPrimitive {
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
    UnitVariant(String),
}

impl From<Primitive> for AstPrimitive {
    fn from(value: Primitive) -> Self {
        match value {
            Primitive::Null => AstPrimitive::Null,
            Primitive::Bool(v) => AstPrimitive::Bool(v),
            Primitive::PosInt(v) => AstPrimitive::PosInt(v),
            Primitive::NegInt(v) => AstPrimitive::NegInt(v),
            Primitive::Float(v) => AstPrimitive::Float(v),
            Primitive::String(v) => AstPrimitive::String(v),
            Primitive::UnitVariant(v) => AstPrimitive::UnitVariant(v),
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
                            Primitive(PosInt(1)),
                            Primitive(PosInt(2)),
                            Primitive(PosInt(3))
                        ])
                    })
                )
            ])
        })
    }

    ok! {
        "> = `Hello`" => Primitive(UnitVariant("Hello"))
    }

    ok! {
        "> foo_bar = `Hello`" =>
        Struct({
            "foo_bar": Primitive(UnitVariant("Hello"))
        })
    }

    ok! {
        "> ['>'] = `Hello`" =>
        Map({
            ">": Primitive(UnitVariant("Hello"))
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
                Primitive(Bool(true))
            )
        )
    }

    ok! {
        ">   foo   >bar> \n>baz = true" =>
        Struct({
            "foo": Struct({
                "bar": Struct({
                    "baz": Primitive(Bool(true))
                })
            })
        })
    }

    ok! {
        ">foo>\n>bar>\n>baz=true" =>
        Struct({
            "foo": Struct({
                "bar": Struct({
                    "baz": Primitive(Bool(true))
                })
            })
        })
    }

    ok! {
        ">foo=42" =>
        Struct({
            "foo": Primitive(PosInt(42))
        })
    }

    ok! {
        "> foo      =      42" =>
        Struct({
            "foo": Primitive(PosInt(42))
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43
        " =>
        Struct({
            "foo": Primitive(PosInt(42)),
            "bar": Primitive(PosInt(43))
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43" =>
        Struct({
            "foo": Primitive(PosInt(42)),
            "bar": Primitive(PosInt(43))
        })
    }

    ok! {
        "
            > foo = 42

            > bar = 43  " =>
        Struct({
            "foo": Primitive(PosInt(42)),
            "bar": Primitive(PosInt(43))
        })
    }

    ok! {
        "

    
            > foo = 42

            > bar = 43


        " =>
        Struct({
            "foo": Primitive(PosInt(42)),
            "bar": Primitive(PosInt(43))
        })
    }

    ok! {
        "> foo  \n =  42" =>
        Struct({
            "foo": Primitive(PosInt(42))
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
            Primitive(Float(1.3e+10)),
            Primitive(String("foo")),
            Primitive(Bool(true)),
            Primitive(PosInt(42)),
        ])
    }

    ok! {
        "> = [ 1\n  ]" =>
        Sequence([
            Primitive(PosInt(1)),
        ])
    }

    ok! {
        "> = [\n1]" =>
        Sequence([
            Primitive(PosInt(1)),
        ])
    }

    ok! {
        "> = [1,\n]" =>
        Sequence([
            Primitive(PosInt(1)),
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
    ok! { "> foo_bar = null" => Struct({ "foo_bar": Primitive(Null) }) }
    ok! { "> `FooBar` = null" => Variant("FooBar", Primitive(Null)) }
    ok! { "> [0] = null" => Sequence([Primitive(Null)]) }
    ok! { "> [\"foobar\"] = null" => Map({ "foobar": Primitive(Null) }) }
    ok! { "> ['foobar'] = null" => Map({ "foobar": Primitive(Null) }) }
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
            "enum_variant": Primitive(UnitVariant("Foo")),
            "seq_of_primitives": Sequence([
                Primitive(String("foo")),
                Primitive(String("bar")),
                Primitive(PosInt(1)),
                Primitive(Float(2.3e1)),
                Primitive(Null)
            ]),
            "null": Primitive(Null),
            "bool": Struct({
                "true": Primitive(Bool(true)),
                "false": Primitive(Bool(false))
            }),
            "pos_int": Struct({
                "dec": Primitive(PosInt(42)),
                "hex": Struct({
                    "hi": Primitive(PosInt(42)),
                    "lo": Primitive(PosInt(42))
                })
            }),
            "float": Sequence([
                Primitive(Float(42.0)),
                Primitive(Float(42.42)),
                Primitive(Float(1.956e-10)),
            ]),
            "string": Struct({
                "raw": Primitive(String(" foo bar ")),
                "double": Primitive(String(" foo bar ")),
                "single": Primitive(String(" foo bar "))
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
            "foo": Primitive(PosInt(3)),
            "bar": Primitive(PosInt(4))
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
            "foo": Primitive(String("foo\nbar\n\nbaz")),
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
