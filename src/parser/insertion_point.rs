use super::error::{parse_error, IntoParseResult, ParseResult};
use super::imp::Node;
use super::path_item::PathItem;
use super::{Parser, Span};
use crate::value::{Value, ValueCell};

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
            let next = path_item.index_value(&host).into_parse_result(span)?;

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
            (Value::Sequence(seq), PathItem::Index(idx)) => {
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
            (Value::Struct(fields), PathItem::FieldName(name)) => {
                fields.insert(name.to_string(), new_value);
            }
            (Value::Map(map), PathItem::MapKey(key)) => {
                map.insert(key, new_value);
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
