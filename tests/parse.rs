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
        let actual = AstValue::from(parse($input).unwrap());
        let expected = ron::from_str::<AstValue>(stringify!( $($expected)+)).unwrap();

        assert_eq!(actual, expected);
    }};
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
        "> `Hello` >    \n> `World` = true" =>
        Variant(
            "Hello",
            Variant(
                "World",
                Primitive(Bool(true))
            )
        )
    }

    ok! {
        "> ['>'] = `Hello`" =>
        Map({
            ">": Primitive(UnitVariant("Hello"))
        })
    }
}
