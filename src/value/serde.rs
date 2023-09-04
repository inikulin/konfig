use super::Value;

#[cfg(feature = "serde")]
impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(v) => serializer.serialize_bool(v),
            Value::Float(v) => serializer.serialize_f64(v),
            Value::UInt(v) => serializer.serialize_u64(v),
            Value::Int(v) => serializer.serialize_i64(v),
            Value::String(ref v) | Value::UnitVariant(ref v) => {
                serializer.serialize_str(v.as_str())
            }
            Value::Map(ref m) | Value::Struct(ref m) => {
                let mut map = serializer.serialize_map(Some(m.len()))?;

                for (k, v) in m {
                    serde::ser::SerializeMap::serialize_entry(&mut map, k, &**v)?;
                }

                serde::ser::SerializeMap::end(map)
            }

            Value::Sequence(ref s) => {
                let mut seq = serializer.serialize_seq(Some(s.len()))?;

                for v in s {
                    serde::ser::SerializeSeq::serialize_element(&mut seq, &**v)?;
                }

                serde::ser::SerializeSeq::end(seq)
            }
            Value::Variant(ref n, ref v) => {
                let mut map = serializer.serialize_map(Some(1))?;

                serde::ser::SerializeMap::serialize_entry(&mut map, n, &**v)?;
                serde::ser::SerializeMap::end(map)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        macro_rules! ok {
            ($input:expr => $($expected:tt)+) => {{
                let actual = serde_json::to_value(&$input).unwrap();
                let expected = serde_json::json!($($expected)+);

                assert_eq!(actual, expected);
            }};
        }

        ok! { Value::Null => null }
        ok! { Value::Bool(true) => true }
        ok! { Value::Bool(false) => false }
        ok! { Value::Float(42.42e+3) => 42.42e+3 }
        ok! { Value::Int(-42) => -42 }
        ok! { Value::UInt(42) => 42 }
        ok! { Value::String("hello".into()) => "hello" }
        ok! { Value::UnitVariant("Hello".into()) => "Hello" }

        ok! {
            Value::Map([
                ("foo".into(), Value::Bool(true).into()),
                ("bar".into(), Value::String("baz".into()).into())
            ].into_iter().collect()) => {
                "foo": true,
                "bar": "baz"
            }
        }

        ok! {
            Value::Struct([
                ("foo".into(), Value::UInt(42).into()),
                ("bar".into(), Value::Int(-42).into())
            ].into_iter().collect()) => {
                "foo": 42,
                "bar": -42
            }
        }

        ok! {
            Value::Sequence(vec![
                Value::String("hello".into()).into(),
                Value::Null.into(),
                Value::Sequence(vec![Value::UInt(42).into()]).into()
            ]) => ["hello", null, [42]]
        }

        ok! {
            Value::Variant("foo".to_string(), Value::UInt(42).into()) =>
            { "foo": 42 }
        }
    }
}
