use super::error::{parse_error, IntoParseResult, ParseResult};
use super::insertion_point::{path_item_to_value, InsertionPoint};
use super::Context;
use crate::value::{PathItem, Value, ValueCell};
use pest_consume::{match_nodes, Parser as PestParser};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) type Node<'i> = pest_consume::Node<'i, Rule, Rc<RefCell<Context>>>;

#[derive(PestParser)]
#[grammar = "./parser/grammar.pest"]
pub(super) struct Parser;

#[pest_consume::parser]
impl Parser {
    pub(super) fn boolean(node: Node) -> ParseResult<bool> {
        Ok(match node.as_str() {
            "true" => true,
            "false" => false,
            _ => unreachable!(),
        })
    }

    #[inline]
    pub(super) fn null(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    pub(super) fn primitive(node: Node) -> ParseResult<ValueCell> {
        Ok(match_nodes! {
            node.children();
            [null(_)] => Value::Null,
            [boolean(v)] => Value::Bool(v),
            [pos_int(v)] => Value::UInt(v),
            [neg_int(v)] => Value::Int(v),
            [float(v)] => Value::Float(v),
            [single_quoted_string(v)] => Value::String(v),
            [double_quoted_string(v)] => Value::String(v),
            [raw_string(v)] => Value::String(v),
            [enum_variant(v)] => Value::UnitVariant(v.to_string()),
        })
        .map(Into::into)
    }

    pub(super) fn pos_int(node: Node) -> ParseResult<u64> {
        let digits = node.children().single().unwrap();

        let radix = match digits.as_rule() {
            Rule::dec_digits => 10,
            Rule::hex_digits => 16,
            _ => unreachable!(),
        };

        u64::from_str_radix(digits.as_str(), radix).into_parse_result(node.as_span())
    }

    pub(super) fn neg_int(node: Node) -> ParseResult<i64> {
        let u64_repr = match_nodes! {
            node.children();
            [pos_int(i)] => i,
        };

        0i64.checked_sub_unsigned(u64_repr)
            .ok_or_else(|| parse_error!(node.as_span(), "number too small to fit in target type"))
    }

    pub(super) fn float(node: Node) -> ParseResult<f64> {
        node.as_str().parse().into_parse_result(node.as_span())
    }

    #[inline]
    pub(super) fn double_quoted_string(node: Node) -> ParseResult<String> {
        parse_quoted_string(node, Rule::double_quoted_string_text)
    }

    #[inline]
    pub(super) fn single_quoted_string(node: Node) -> ParseResult<String> {
        parse_quoted_string(node, Rule::single_quoted_string_text)
    }

    #[inline]
    pub(super) fn raw_string_start(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    #[inline]
    pub(super) fn raw_string_end(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    #[inline]
    pub(super) fn raw_string_text(node: Node) -> ParseResult<String> {
        Ok(node.as_str().to_string())
    }

    pub(super) fn raw_string(node: Node) -> ParseResult<String> {
        Ok(match_nodes! {
            node.children();
            [raw_string_start(_), raw_string_text(t), raw_string_end(_)] => t,
        })
    }

    pub(super) fn esc(node: Node) -> ParseResult<char> {
        Ok(match_nodes! {
            node.children();
            [esc_alias(c)] => c,
            [esc_unicode(c)] => c,
        })
    }

    pub(super) fn esc_alias(node: Node) -> ParseResult<char> {
        Ok(match node.as_str() {
            "\"" => '"',
            "\\" => '\\',
            "/" => '/',
            "b" => '\x08',
            "f" => '\x0C',
            "n" => '\n',
            "r" => '\r',
            "t" => '\t',
            _ => unreachable!(),
        })
    }

    pub(super) fn esc_unicode(node: Node) -> ParseResult<char> {
        let code_point =
            u32::from_str_radix(node.as_str(), 16).into_parse_result(node.as_span())?;

        char::try_from(code_point).into_parse_result(node.as_span())
    }

    pub(super) fn array_of_primitives(node: Node) -> ParseResult<ValueCell> {
        match node.children().single() {
            Ok(values) => parse_seq_of_primitives(&mut values.children()),
            _ => parse_seq_of_primitives(&mut [].into_iter()),
        }
    }

    #[inline]
    pub(super) fn list_of_primitives(node: Node) -> ParseResult<ValueCell> {
        parse_seq_of_primitives(&mut node.children())
    }

    pub(super) fn rhs(node: Node) -> ParseResult<ValueCell> {
        let value = match_nodes! {
            node.children();
            [primitive(v)] => v,
            [array_of_primitives(s)] => s,
            [list_of_primitives(s)] => s
        };

        let mut ctx = node.user_data().borrow_mut();

        ctx.last_rhs = Some(value.rc_clone());

        value.borrow_mut().lexical_info.docs_before = ctx.pending_docs.take().unwrap_or_default();

        Ok(value)
    }

    pub(super) fn enum_variant(node: Node) -> ParseResult<&str> {
        Ok(node.children().single().unwrap().as_str())
    }

    pub(super) fn index(node: Node) -> ParseResult<usize> {
        node.children()
            .single()
            .unwrap()
            .as_str()
            .parse()
            .into_parse_result(node.as_span())
    }

    pub(super) fn field_name(node: Node) -> ParseResult<&str> {
        Ok(node.as_str())
    }

    pub(super) fn map_key(node: Node) -> ParseResult<String> {
        Ok(match_nodes! {
            node.children().single().unwrap().children();
            [single_quoted_string(k)] => k,
            [double_quoted_string(k)] => k,
        })
    }

    pub(super) fn path_item(node: Node) -> ParseResult<PathItem> {
        Ok(match_nodes! {
            node.children();
            [field_name(n)] => PathItem::StructFieldName(n.into()),
            [enum_variant(v)] => PathItem::VariantName(v.into()),
            [map_key(k)] => PathItem::MapKey(k.into()),
            [index(i)] => PathItem::SequenceIndex(i)
        })
    }

    #[inline]
    pub(super) fn path(node: Node) -> ParseResult<Node> {
        Ok(node)
    }

    #[inline]
    fn EOI(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    #[inline]
    fn expr_terminator(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    pub(super) fn expr(node: Node) -> ParseResult<()> {
        let span = node.as_span();
        let ctx = node.user_data();

        let (mut path, mut new_value) = match_nodes! {
            node.children();
            [path(p), rhs(r), expr_terminator(_)] => (
                p.into_children().filter(|n| n.as_rule() == Rule::path_item),
                r
            ),
        };

        let insertion_point = ctx
            .borrow()
            .root
            .as_ref()
            .map(|root| InsertionPoint::find(&mut path, span, root.rc_clone()))
            .transpose()?;

        for node in path.rev() {
            let span = node.as_span();
            let path_item = Parser::path_item(node)?;

            new_value = path_item_to_value(path_item, new_value, span)?;
        }

        match insertion_point {
            Some(insertion_point) => insertion_point.insert(new_value)?,
            None => ctx.borrow_mut().root = Some(new_value),
        }

        Ok(())
    }

    pub(super) fn konfig(node: Node) -> ParseResult<()> {
        node.into_children()
            .filter(|node| match node.as_rule() {
                Rule::expr => true,
                Rule::docs => {
                    let ctx = node.user_data();

                    ctx.borrow_mut()
                        .pending_docs
                        .get_or_insert_with(String::new)
                        .push_str(node.as_str());

                    false
                }
                _ => false,
            })
            .try_for_each(Parser::expr)
    }
}

#[allow(clippy::result_large_err)]
fn parse_quoted_string(node: Node, text_rule: Rule) -> ParseResult<String> {
    let mut string = String::default();
    let content = node.children().single().unwrap();

    for node in content.into_children() {
        match node.as_rule() {
            r if r == text_rule => string.push_str(node.as_str()),
            Rule::esc => {
                string.push(Parser::esc(node)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(string)
}

#[allow(clippy::result_large_err)]
pub(super) fn parse_seq_of_primitives<'i>(
    primitive_nodes: &mut impl Iterator<Item = Node<'i>>,
) -> ParseResult<ValueCell> {
    let mut seq = Vec::new();

    for node in primitive_nodes {
        seq.push(Parser::primitive(node)?);
    }

    let value = ValueCell::from(Value::Sequence(seq));

    value.borrow_mut().lexical_info.is_rhs_seq = true;

    Ok(value)
}
