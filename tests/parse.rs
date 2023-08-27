use konfig::parser::parse;
use konfig::value::{Primitive, Value};
use std::collections::HashMap;

macro_rules! map {
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = HashMap::new();

            $(
                m.insert($k.to_string(), $v);
            )+

            m
        }
    };
}

macro_rules! ok {
    ($input:expr => $expected:expr) => {
        assert_eq!(parse($input), Ok($expected));
    };
}

#[test]
fn simple_assignment() {
    ok! {
        "> foo > bar   > baz = 42" =>
        Value::Struct(map!(
            "foo" => Value::Struct(map!(
                "bar" => Value::Struct(map!(
                    "baz" => Value::Primitive(Primitive::PosInt(42)).into()
                )).into()
            )).into()
        )).into()
    };

    ok! {
        "> foo_bar > [0] > `Baz` > ['qux quz'] = [1, 2, 3]" =>
        Value::Struct(map!(
            "foo_bar" => Value::Sequence(vec![
                Value::Variant("Baz".into(),
                    Value::Map(map!(
                        "qux quz" => Value::SequenceOfPrimitives(vec![
                                Primitive::PosInt(1),
                                Primitive::PosInt(2),
                                Primitive::PosInt(3),
                            ]).into()
                    )).into()
                ).into()
            ]).into()
        )).into()
    }

    ok! {
        "> = `Hello`" =>
        Value::Primitive(Primitive::UnitVariant("Hello".into())).into()
    };

    ok! {
        "> foo_bar = `Hello`" =>
        Value::Struct(map!("foo_bar" =>
            Value::Primitive(Primitive::UnitVariant("Hello".into())).into()
        )).into()
    }

    ok! {
        "> `Hello` >    \n> `World` = true" =>
        Value::Variant(
            "Hello".into(),
            Value::Variant(
                "World".into(),
                Value::Primitive(Primitive::Bool(true)).into(),
            )
            .into(),
        ).into()
    }

    ok! {
        "> ['>'] = `Hello`" =>
        Value::Map(map!(">" =>
            Value::Primitive(Primitive::UnitVariant("Hello".into())).into()
        )).into()
    }
}
