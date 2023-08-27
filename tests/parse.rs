use konfig::parser::ast::{Leaf, Node, Value};
use konfig::parser::parse;
use std::collections::HashMap;

macro_rules! map {
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = HashMap::new();

            $(
                m.insert($k, $v);
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
        Node::Fields(map!(
            "foo" => Node::Fields(map!(
                "bar" => Node::Fields(map!(
                    "baz" => Node::Leaf(Leaf::Value(Value::PosInt(42))).into()
                )).into()
            )).into()
        )).into()
    };

    ok! {
        "> foo_bar > [0] > `Baz` > ['qux quz'] = [1, 2, 3]" =>
        Node::Fields(map!(
            "foo_bar" => Node::Sequence(vec![
                Node::NewTypeEnumVariant("Baz",
                    Node::Map(map!(
                        "qux quz".into() => Node::Leaf(
                            Leaf::InlineSequence(vec![
                                Value::PosInt(1),
                                Value::PosInt(2),
                                Value::PosInt(3),
                            ])).into()
                    )).into()
                ).into()
            ]).into()
        )).into()
    }

    ok! {
        "> = `Hello`" =>
        Node::Leaf(Leaf::UnitEnumVariant("Hello")).into()
    };

    ok! {
        "> foo_bar = `Hello`" =>
        Node::Fields(map!("foo_bar" =>
            Node::Leaf(Leaf::UnitEnumVariant("Hello")).into()
        )).into()
    }

    ok! {
        "> `Hello` >    \n> `World` = true" =>
        Node::NewTypeEnumVariant(
            "Hello",
            Node::NewTypeEnumVariant(
                "World",
                Node::Leaf(Leaf::Value(Value::Bool(true))).into(),
            )
            .into(),
        ).into()
    }

    ok! {
        "> ['>'] = `Hello`" =>
        Node::Map(map!(">".into() =>
            Node::Leaf(Leaf::UnitEnumVariant("Hello")).into()
        )).into()
    }
}
