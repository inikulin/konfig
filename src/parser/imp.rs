use super::error::{parse_error, IntoParseResult, ParseResult};
use super::insertion_point::InsertionPoint;
use super::path_item::PathItem;
use super::Ast;
use crate::value::{Primitive, Value};
use pest_consume::{match_nodes, Parser as PestParser};

pub(super) type Node<'i> = pest_consume::Node<'i, Rule, Ast>;

#[derive(PestParser)]
#[grammar = "./parser/grammar.pest"]
pub(super) struct Parser;

#[pest_consume::parser]
impl Parser {
    pub(super) fn boolean(node: Node) -> ParseResult<bool> {
        Ok(match node.children().single().unwrap().as_rule() {
            Rule::boolean_true => true,
            Rule::boolean_false => false,
            _ => unreachable!(),
        })
    }

    #[inline]
    pub(super) fn null(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    pub(super) fn primitive(node: Node) -> ParseResult<Primitive> {
        Ok(match_nodes! {
            node.children();
            [null(_)] => Primitive::Null,
            [boolean(v)] => Primitive::Bool(v),
            [pos_int(v)] => Primitive::PosInt(v),
            [neg_int(v)] => Primitive::NegInt(v),
            [float(v)] => Primitive::Float(v),
            [single_quoted_string(v)] => Primitive::String(v),
            [double_quoted_string(v)] => Primitive::String(v),
            [raw_string(v)] => Primitive::String(v),
            [enum_variant(v)] => Primitive::UnitVariant(v.to_string()),
        })
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

    pub(super) fn esc(node: Node) -> ParseResult<Option<char>> {
        Ok(match_nodes! {
            node.children();
            [esc_alias(c)] => c,
            [esc_unicode(c)] => Some(c),
        })
    }

    pub(super) fn esc_alias(node: Node) -> ParseResult<Option<char>> {
        Ok(match node.as_str() {
            "\"" => Some('"'),
            "\\" => Some('\\'),
            "/" => Some('/'),
            "b" => Some('\x08'),
            "f" => Some('\x0C'),
            "n" => Some('\n'),
            "r" => Some('\r'),
            "t" => Some('\t'),
            "\n" | "\r" | "\r\n" => None,
            _ => unreachable!(),
        })
    }

    pub(super) fn esc_unicode(node: Node) -> ParseResult<char> {
        let code_point =
            u32::from_str_radix(node.as_str(), 16).into_parse_result(node.as_span())?;

        char::try_from(code_point).into_parse_result(node.as_span())
    }

    pub(super) fn sequence_of_primitives(node: Node) -> ParseResult<Vec<Primitive>> {
        let mut seq = Vec::new();

        if let Ok(values) = node.children().single() {
            for node in values.children() {
                seq.push(Parser::primitive(node)?);
            }
        }

        Ok(seq)
    }

    pub(super) fn rhs(node: Node) -> ParseResult<Value> {
        Ok(match_nodes! {
            node.children();
            [primitive(v)] => Value::Primitive(v),
            [sequence_of_primitives(s)] => Value::SequenceOfPrimitives(s),
        })
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
            [field_name(n)] => PathItem::FieldName(n),
            [enum_variant(v)] => PathItem::EnumVariant(v),
            [map_key(k)] => PathItem::MapKey(k),
            [index(i)] => PathItem::Index(i)
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

        for node in node.children() {
            println!("{:#?}", node.as_rule());
        }

        let (mut path, rhs) = match_nodes! {
            node.children();
            [path(p), rhs(r), expr_terminator(_)] => (
                p.into_children().filter(|n| n.as_rule() == Rule::path_item),
                r
            ),
        };

        let mut new_value = rhs.into();
        let mut insertion_point = None;

        if let Some(root) = node.user_data().borrow().as_ref() {
            insertion_point = Some(InsertionPoint::find(
                &mut path,
                &new_value,
                span,
                root.rc_clone(),
            )?);
        }

        for node in path.rev() {
            let span = node.as_span();

            new_value = Parser::path_item(node)?.into_value(new_value, span)?;
        }

        match insertion_point {
            Some(insertion_point) => insertion_point.insert(new_value)?,
            None => *node.user_data().borrow_mut() = Some(new_value),
        }

        Ok(())
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
                if let Some(esc) = Parser::esc(node)? {
                    string.push(esc);
                }
            }
            _ => unreachable!(),
        }
    }

    Ok(string)
}
