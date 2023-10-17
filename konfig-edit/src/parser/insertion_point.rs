use super::error::{parse_error, ParseResult};
use super::imp::Node;
use super::{Parser, Span};
use crate::value::{PathItem, Value, ValueCell};

pub(super) struct InsertionPoint<'i> {
    host: ValueCell,
    path_item: PathItem<'i>,
    span: Span<'i>,
}

impl<'i> InsertionPoint<'i> {
    #[allow(clippy::result_large_err)]
    pub(super) fn find(
        path: &mut impl Iterator<Item = Node<'i>>,
        assignment_span: Span,
        root: ValueCell,
    ) -> ParseResult<InsertionPoint<'i>> {
        let mut host = root;

        for node in path.by_ref() {
            let span = node.as_span();
            let path_item = Parser::path_item(node)?;
            let next = index_value_by_path_item(&path_item, &host, span)?;

            match next {
                Some(next) => host = next,
                None => {
                    return Ok(InsertionPoint {
                        host,
                        path_item,
                        span,
                    })
                }
            }
        }

        Err(parse_error!(
            assignment_span,
            "the path already has a value assigned",
        ))
    }

    #[allow(clippy::result_large_err)]
    pub(super) fn insert(self, new_value: ValueCell) -> ParseResult<()> {
        match (&mut self.host.borrow_mut().value, self.path_item) {
            (Value::Sequence(seq), PathItem::SequenceIndex(idx)) => {
                if idx != seq.len() {
                    return Err(parse_error!(
                        self.span,
                        "sequence items must be defined in order; \
                        last seen item index: {}, specified item index: {}",
                        seq.len().saturating_sub(1),
                        idx
                    ));
                }

                seq.push(new_value);
            }
            (Value::Struct(fields), PathItem::StructFieldName(name)) => {
                fields.insert(name.to_string(), new_value);
            }
            (Value::Map(map), PathItem::MapKey(key)) => {
                map.insert(key.to_string(), new_value);
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}

#[allow(clippy::result_large_err)]
pub(super) fn path_item_to_value(
    path_item: PathItem,
    prev: ValueCell,
    span: Span,
) -> ParseResult<ValueCell> {
    match path_item {
        PathItem::SequenceIndex(0) => Ok(Value::Sequence(vec![prev])),
        PathItem::SequenceIndex(_) => Err(parse_error!(
            span,
            "sequence items should be defined in order, with the first item having index `0`"
        )),
        PathItem::MapKey(key) => Ok(Value::Map([(key.to_string(), prev)].into_iter().collect())),
        PathItem::StructFieldName(name) => Ok(Value::Struct(
            [(name.to_string(), prev)].into_iter().collect(),
        )),
        PathItem::VariantName(variant) => Ok(Value::Variant(variant.to_string(), prev)),
    }
    .map(Into::into)
}

#[allow(clippy::result_large_err)]
fn index_value_by_path_item(
    path_item: &PathItem,
    value_cell: &ValueCell,
    span: Span,
) -> ParseResult<Option<ValueCell>> {
    let value_cell_ref = value_cell.borrow();

    match (path_item, &value_cell_ref.value) {
        (PathItem::SequenceIndex(idx), Value::Sequence(seq))
            if !value_cell_ref.lexical_info.is_rhs_seq =>
        {
            Ok(seq.get(*idx).map(ValueCell::rc_clone))
        }
        (PathItem::MapKey(key), Value::Map(map)) => {
            Ok(map.get::<str>(key).map(ValueCell::rc_clone))
        }
        (PathItem::StructFieldName(name), Value::Struct(fields)) => {
            Ok(fields.get::<str>(name).map(ValueCell::rc_clone))
        }
        (PathItem::VariantName(var1), Value::Variant(var2, value)) if var1 == var2 => {
            Ok(Some(value.rc_clone()))
        }
        _ => Err(parse_error!(
            span,
            "path item has incompatible type with the previously specified values"
        )),
    }
}
