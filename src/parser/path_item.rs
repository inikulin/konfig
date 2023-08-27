use super::error;
use super::imp::ParseResult;
use super::type_name::TypeName;
use crate::value::{Value, ValueCell};
use pest::Span;

#[derive(Debug, PartialEq)]
pub(super) enum PathItem<'i> {
    Index(usize),
    MapKey(String),
    FieldName(&'i str),
    EnumVariant(&'i str),
}

impl<'i> PathItem<'i> {
    #[allow(clippy::result_large_err)]
    pub(super) fn into_value(self, prev: ValueCell, span: Span) -> ParseResult<ValueCell> {
        match self {
            PathItem::Index(0) => Ok(Value::Sequence(vec![prev])),
            PathItem::Index(_) => Err(error!(
                span,
                "sequence items should be defined in order, with the first item having index `0`"
            )),
            PathItem::MapKey(key) => Ok(Value::Map([(key, prev)].into_iter().collect())),
            PathItem::FieldName(name) => Ok(Value::Struct(
                [(name.to_string(), prev)].into_iter().collect(),
            )),
            PathItem::EnumVariant(variant) => Ok(Value::Variant(variant.to_string(), prev)),
        }
        .map(Into::into)
    }

    pub(super) fn index_value(&self, value: &Value) -> Result<Option<ValueCell>, String> {
        match (self, value) {
            (PathItem::Index(idx), Value::Sequence(seq)) => {
                Ok(seq.get(*idx).map(ValueCell::rc_clone))
            }
            (PathItem::MapKey(key), Value::Map(map)) => Ok(map.get(key).map(ValueCell::rc_clone)),
            (PathItem::FieldName(name), Value::Struct(fields)) => {
                Ok(fields.get(*name).map(ValueCell::rc_clone))
            }
            (PathItem::EnumVariant(var1), Value::Variant(var2, value)) if var1 == var2 => {
                Ok(Some(value.rc_clone()))
            }
            _ => Err(format!(
                "path item is expected to be {}, but it was previously defined as {}",
                self.type_name(),
                value.type_name()
            )),
        }
    }
}