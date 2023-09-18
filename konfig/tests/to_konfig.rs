use konfig::{Error, Value};

#[test]
fn serialization_with_docs() {
    let src = include_str!("./data/doc_parsing.konfig.md");
    let parsed = konfig::parse(src).unwrap();
    let serialized = parsed.to_konfig().unwrap();

    assert_eq!(serialized, src);
}

#[test]
fn invalid_identifiers() {
    let err = Value::Variant("123".into(), Value::Null.into())
        .into_cell()
        .to_konfig()
        .unwrap_err();

    assert_eq!(err, Error::InvalidFieldNameOrEnumVariant("123".into()));

    let err = Value::Struct(
        [("foo bar".into(), Value::Null.into())]
            .into_iter()
            .collect(),
    )
    .into_cell()
    .to_konfig()
    .unwrap_err();

    assert_eq!(err, Error::InvalidFieldNameOrEnumVariant("foo bar".into()));
}
