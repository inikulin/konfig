use super::path_item::PathItem;
use crate::value::{Primitive, Value};
use std::borrow::Cow;

pub(super) trait TypeName {
    fn type_name(&self) -> Cow<'static, str>;
}

impl TypeName for PathItem<'_> {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            PathItem::EnumVariant(v) => format!("new type enum variant `{v}`").into(),
            PathItem::FieldName(_) => "structure".into(),
            PathItem::Index(_) => "sequence".into(),
            PathItem::MapKey(_) => "map".into(),
        }
    }
}

impl TypeName for Value {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            Value::Struct(_) => "structure".into(),
            Value::Map(_) => "map".into(),
            Value::Variant(v, _) => format!("new type enum variant `{v}`").into(),
            Value::Sequence(_) => "sequence".into(),
            Value::SequenceOfPrimitives(_) => "inline sequence".into(),
            Value::Primitive(v) => v.type_name(),
        }
    }
}

impl TypeName for Primitive {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            Primitive::Bool(_) => "boolean value".into(),
            Primitive::Float(_) => "floating point number value".into(),
            Primitive::NegInt(_) => "negative integer value".into(),
            Primitive::PosInt(_) => "positive integer value".into(),
            Primitive::Null => "null value".into(),
            Primitive::String(_) => "string value".into(),
            Primitive::UnitVariant(v) => format!("unit enum variant `{v}`").into(),
        }
    }
}
