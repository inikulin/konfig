use super::{ast, PathItem};
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

impl TypeName for ast::Node {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            ast::Node::Fields(_) => "structure".into(),
            ast::Node::Map(_) => "map".into(),
            ast::Node::NewTypeEnumVariant(v, _) => format!("new type enum variant `{v}`").into(),
            ast::Node::Sequence(_) => "sequence".into(),
            ast::Node::Leaf(l) => l.type_name(),
        }
    }
}

impl TypeName for ast::Leaf {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            ast::Leaf::InlineSequence(_) => "inline sequence".into(),
            ast::Leaf::UnitEnumVariant(v) => format!("unit enum variant `{v}`").into(),
            ast::Leaf::Value(v) => v.type_name(),
        }
    }
}

impl TypeName for ast::Value {
    fn type_name(&self) -> Cow<'static, str> {
        match self {
            ast::Value::Bool(_) => "boolean value",
            ast::Value::Float(_) => "floating point number value",
            ast::Value::NegInt(_) => "negative integer value",
            ast::Value::PosInt(_) => "positive integer value",
            ast::Value::Null => "null value",
            ast::Value::String(_) => "string value",
        }
        .into()
    }
}
