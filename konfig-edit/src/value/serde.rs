use super::Value;
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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

impl<'de> serde::Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(ValueVisitor)
    }
}

struct ValueVisitor;

impl<'de> serde::de::Visitor<'de> for ValueVisitor {
    type Value = Value;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("any valid konfig value")
    }

    #[inline]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(v))
    }

    #[inline]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Value::Int(v))
    }

    #[inline]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Value::UInt(v))
    }

    #[inline]
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Self::Value, E> {
        if v.is_finite() {
            Ok(Value::Float(v))
        } else {
            Err(serde::de::Error::custom(Error::InfAndNanNotSupported))
        }
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Value::String(v.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(Value::String(v))
    }

    #[inline]
    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    #[inline]
    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        serde::de::Deserialize::deserialize(deserializer)
    }

    #[inline]
    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut vec = Vec::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some(elem) = visitor.next_element::<Value>()? {
            vec.push(elem.into());
        }

        Ok(Value::Sequence(vec))
    }

    fn visit_map<A>(self, mut visitor: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut map = HashMap::with_capacity(visitor.size_hint().unwrap_or(0));

        while let Some((key, value)) = visitor.next_entry::<String, Value>()? {
            map.insert(key, value.into());
        }

        Ok(Value::Map(map))
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
            ].into_iter().collect())

            =>

            {
                "foo": true,
                "bar": "baz"
            }
        }

        ok! {
            Value::Struct([
                ("foo".into(), Value::UInt(42).into()),
                ("bar".into(), Value::Int(-42).into())
            ].into_iter().collect())

            =>

            {
                "foo": 42,
                "bar": -42
            }
        }

        ok! {
            Value::Sequence(vec![
                Value::String("hello".into()).into(),
                Value::Null.into(),
                Value::Sequence(vec![Value::UInt(42).into()]).into()
            ])

            =>

            ["hello", null, [42]]
        }

        ok! {
            Value::Variant("foo".to_string(), Value::UInt(42).into())

            =>

            { "foo": 42 }
        }
    }

    #[test]
    fn deserialize() {
        use serde_json::json;

        macro_rules! ok {
            ($input:expr => $expected:expr) => {{
                let actual: Value = serde_json::from_value(serde_json::json!($input)).unwrap();

                assert_eq!(actual, $expected);
            }};
        }

        ok! { json!(null) => Value::Null }
        ok! { json!(true) => Value::Bool(true) }
        ok! { json!(false) => Value::Bool(false) }
        ok! { json!(42.42e+3) =>  Value::Float(42.42e+3) }
        ok! { json!(-42) => Value::Int(-42) }
        ok! { json!(42) => Value::UInt(42) }
        ok! { json!("hello") => Value::String("hello".into()) }

        ok! {
            json!({
                "foo": true,
                "bar": "baz"
            })

            =>

            Value::Map([
                ("foo".into(), Value::Bool(true).into()),
                ("bar".into(), Value::String("baz".into()).into())
            ].into_iter().collect())
        }

        ok! {
            json!(["hello", null, [42]])

            =>

            Value::Sequence(vec![
                Value::String("hello".into()).into(),
                Value::Null.into(),
                Value::Sequence(vec![Value::UInt(42).into()]).into()
            ])
        }
    }
}
