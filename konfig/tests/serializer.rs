use indoc::indoc;
use konfig::{Error, Value};

#[test]
fn serialization_with_docs() {
    let src = include_str!("./data/doc_parsing.konfig.md");
    let parsed = konfig::parse(src).unwrap();
    let serialized = konfig::serialize(&parsed, Default::default()).unwrap();

    assert_eq!(serialized, src);
}

#[test]
fn invalid_identifiers() {
    let v = Value::Variant("123".into(), Value::Null.into()).into_cell();

    assert_eq!(
        konfig::serialize(&v, Default::default()).unwrap_err(),
        Error::InvalidFieldNameOrEnumVariant("123".into())
    );

    let v = Value::Struct(
        [("foo bar".into(), Value::Null.into())]
            .into_iter()
            .collect(),
    )
    .into_cell();

    assert_eq!(
        konfig::serialize(&v, Default::default()).unwrap_err(),
        Error::InvalidFieldNameOrEnumVariant("foo bar".into())
    );
}

#[test]
fn docs_escaping() {
    macro_rules! ok {
        (before: $before:expr, after: $after:expr, expected: $expected:expr) => {
            let mut value = Value::Null.into_cell();

            value.lexical_info_mut().docs_before = indoc!($before).to_string();
            value.lexical_info_mut().docs_after = indoc!($after).to_string();

            let serialialized = konfig::serialize(&value, Default::default()).unwrap();

            assert_eq!(serialialized, indoc!($expected));
        };
    }

    ok! {
        before: "    > hello",
        after: "    > bye",
        expected: "
            \\> hello
        > = null

            \\> bye
        "
    }

    ok! {
        before: "
            foo

        > hello
        
        bar",
        after: "> bye",
        expected: "
            foo

        \\> hello
        
        bar
        > = null

        \\> bye
        "
    }

    ok! {
        before: "
            hello

            world
        baz
        ",
        after: "
        
        > bye
        
        ",
        expected: "
            hello

            world
        baz
        > = null


        \\> bye

        "
    }
}
