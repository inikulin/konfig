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
    SequenceOfPrimitives(Vec<AstPrimitive>),
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
            Value::SequenceOfPrimitives(s) => {
                AstValue::SequenceOfPrimitives(s.into_iter().map(Into::into).collect())
            }
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
        assert_eq!(
            parse($input).unwrap_err().to_string(),
            indoc::indoc!($expected)
        );
    };
}

// TODO remove
#[allow(unused_macros)]
macro_rules! print_err {
    ($input:expr) => {
        println!("{}", parse($input).unwrap_err().to_string());
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
                        "qux quz" : SequenceOfPrimitives([PosInt(1), PosInt(2), PosInt(3)])
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
        "> foo   \n   =   \n   42" =>
        Struct({
            "foo": Primitive(PosInt(42))
        })
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
        " --> 2:2
        |
      2 |  ␊
        |  ^---
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
        SequenceOfPrimitives([
            Float(1.3e+10),
            String("foo"),
            Bool(true),
            PosInt(42),
        ])
    }

    ok! {
        "> = [ 1\n  ]" =>
        SequenceOfPrimitives([
            PosInt(1),
        ])
    }

    ok! {
        "> = [\n1]" =>
        SequenceOfPrimitives([
            PosInt(1),
        ])
    }

    ok! {
        "> = [1,\n]" =>
        SequenceOfPrimitives([
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
