use super::imp::{IntoParseResult, Node, ParseResult};
use super::path_item::PathItem;
use super::type_name::TypeName;
use super::{error, Parser, Span};
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
        rhs: &ValueCell,
        assignment_span: Span,
        root: ValueCell,
    ) -> ParseResult<InsertionPoint<'i>> {
        let mut host = root;

        for node in path.by_ref() {
            let span = node.as_span();
            let path_item = Parser::path_item(node)?;

            let next = path_item
                .index_value(&host.borrow())
                .into_parse_result(span)?;

            match next {
                Some(next) => host = next,
                None => {
                    return if host.borrow().is_multitenant() {
                        Ok(InsertionPoint {
                            host,
                            path_item,
                            span,
                        })
                    } else {
                        Err(error!(
                            assignment_span,
                            "attempt to assign {} to the path that was assigned {} previously",
                            rhs.borrow().type_name(),
                            host.borrow().type_name()
                        ))
                    }
                }
            }
        }

        Err(error!(
            assignment_span,
            "attempt to assign {} to a path item that was previously defined as {}",
            rhs.borrow().type_name(),
            host.borrow().type_name()
        ))
    }

    #[allow(clippy::result_large_err)]
    pub(super) fn insert(self, new_value: ValueCell) -> ParseResult<()> {
        match (&mut *self.host.borrow_mut(), self.path_item) {
            (Value::Sequence(seq), PathItem::Index(idx)) => {
                if idx != seq.len() {
                    return Err(error!(
                        self.span,
                        "sequence items must be defined in order; \
                        current sequence length: {}, specified item index: {}",
                        seq.len(),
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
