use super::imp::{IntoParseResult, Node, ParseResult};
use super::type_name::TypeName;
use super::{ast, error, Parser, PathItem, Span};

pub(super) struct InsertionPoint<'i> {
    host_node: ast::NodeCell,
    path_item: PathItem<'i>,
    span: Span<'i>,
}

impl<'i> InsertionPoint<'i> {
    #[allow(clippy::result_large_err)]
    pub(super) fn find(
        path: &mut impl Iterator<Item = Node<'i>>,
        rhs: &ast::NodeCell,
        assignment_span: Span,
        ast: ast::NodeCell,
    ) -> ParseResult<InsertionPoint<'i>> {
        let mut host_node = ast;

        for node in path.by_ref() {
            let span = node.as_span();
            let path_item = Parser::path_item(node)?;
            let next = host_node.borrow().get(&path_item).into_parse_result(span)?;

            match next {
                Some(next) => host_node = next,
                None => {
                    return if host_node.borrow().is_multitenant() {
                        Ok(InsertionPoint {
                            host_node,
                            path_item,
                            span,
                        })
                    } else {
                        Err(error!(
                            assignment_span,
                            "attempt to assign {} to the path that was assigned {} previously",
                            rhs.borrow().type_name(),
                            host_node.borrow().type_name()
                        ))
                    }
                }
            }
        }

        Err(error!(
            assignment_span,
            "attempt to assign {} to a path item that was previously defined as {}",
            rhs.borrow().type_name(),
            host_node.borrow().type_name()
        ))
    }

    #[allow(clippy::result_large_err)]
    pub(super) fn insert(self, new_node: ast::NodeCell) -> ParseResult<()> {
        match (&mut *self.host_node.borrow_mut(), self.path_item) {
            (ast::Node::Sequence(seq), PathItem::Index(idx)) => {
                if idx != seq.len() {
                    return Err(error!(
                        self.span,
                        "sequence items must be defined in order; \
                        current sequence length: {}, specified item index: {}",
                        seq.len(),
                        idx
                    ));
                }

                seq.push(new_node);
            }
            (ast::Node::Fields(fields), PathItem::FieldName(name)) => {
                fields.insert(name.to_string(), new_node);
            }
            (ast::Node::Map(map), PathItem::MapKey(key)) => {
                map.insert(key, new_node);
            }
            _ => unreachable!(),
        }

        Ok(())
    }
}
