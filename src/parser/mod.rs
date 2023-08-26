use crate::error::{Error, Result};
use pest::error::ErrorVariant;
use pest::Span;
use pest_consume::{match_nodes, Error as PestError, Parser as PestParser};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type ParseResult<T> = std::result::Result<T, PestError<Rule>>;
pub type Context<'i> = Rc<RefCell<Option<AstNode<'i>>>>;
pub type Node<'i> = pest_consume::Node<'i, Rule, Context<'i>>;

macro_rules! error {
    ($span:expr, $msg:literal) => {
        error!($span, $msg,)
    };
    ($span:expr, $msg:literal, $($arg:expr),*) => {
        PestError::new_from_span(ErrorVariant::CustomError {
            message: format!($msg, $($arg),*),
        }, $span.clone())
    }
}

macro_rules! map {
    ($($k:expr => $v:expr),+) => {
        {
            let mut m = HashMap::new();

            $(
                m.insert($k, $v);
            )+

            m
        }
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseError(Box<PestError<Rule>>);

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, PartialEq)]
pub enum AstNode<'i> {
    Sequence(Vec<AstNode<'i>>),
    Map(HashMap<String, AstNode<'i>>),
    NewTypeEnumVariant(&'i str, Box<AstNode<'i>>),
    Fields(HashMap<&'i str, AstNode<'i>>),
    Leaf(AstLeaf<'i>),
}

#[derive(Debug, PartialEq)]
pub enum AstLeaf<'i> {
    InlineSequence(Vec<Value>),
    UnitEnumVariant(&'i str),
    Value(Value),
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    PosInt(u64),
    NegInt(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
enum PathItem<'i> {
    Index(usize),
    MapKey(String),
    FieldName(&'i str),
    EnumVariant(&'i str),
}

impl<'i> PathItem<'i> {
    #[allow(clippy::result_large_err)]
    fn into_ast_node(self, prev: AstNode<'i>, span: Span<'i>) -> ParseResult<AstNode<'i>> {
        match self {
            PathItem::Index(0) => Ok(AstNode::Sequence(vec![prev])),
            PathItem::Index(_) => Err(error!(
                span,
                "sequence items should be defined in order, with the first item having index `0`"
            )),
            PathItem::MapKey(key) => Ok(AstNode::Map(map!(key => prev))),
            PathItem::FieldName(name) => Ok(AstNode::Fields(map!(name => prev))),
            PathItem::EnumVariant(variant) => Ok(AstNode::NewTypeEnumVariant(variant, prev.into())),
        }
    }
}

#[derive(PestParser)]
#[grammar = "./parser/grammar.pest"]
struct Parser;

trait IntoParseResult<T> {
    #[allow(clippy::result_large_err)]
    fn into_parse_result(self, span: Span) -> ParseResult<T>;
}

impl<T, E> IntoParseResult<T> for std::result::Result<T, E>
where
    E: ToString,
{
    #[inline]
    fn into_parse_result(self, span: Span) -> ParseResult<T> {
        self.map_err(|e| error!(span, "{}", e.to_string()))
    }
}

#[pest_consume::parser]
impl Parser {
    fn boolean(node: Node) -> ParseResult<bool> {
        Ok(match node.children().single().unwrap().as_rule() {
            Rule::boolean_true => true,
            Rule::boolean_false => false,
            _ => unreachable!(),
        })
    }

    #[inline]
    fn null(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    fn value(node: Node) -> ParseResult<Value> {
        Ok(match_nodes! {
            node.children();
            [null(_)] => Value::Null,
            [boolean(v)] => Value::Bool(v),
            [pos_int(v)] => Value::PosInt(v),
            [neg_int(v)] => Value::NegInt(v),
            [float(v)] => Value::Float(v),
            [single_quoted_string(v)] => Value::String(v),
            [double_quoted_string(v)] => Value::String(v),
            [raw_string(v)] => Value::String(v)
        })
    }

    fn pos_int(node: Node) -> ParseResult<u64> {
        let digits = node.children().single().unwrap();

        let radix = match digits.as_rule() {
            Rule::dec_digits => 10,
            Rule::hex_digits => 16,
            _ => unreachable!(),
        };

        u64::from_str_radix(digits.as_str(), radix).into_parse_result(node.as_span())
    }

    fn neg_int(node: Node) -> ParseResult<i64> {
        let u64_repr = match_nodes! {
            node.children();
            [pos_int(i)] => i,
        };

        0i64.checked_sub_unsigned(u64_repr)
            .ok_or_else(|| error!(node.as_span(), "number too small to fit in target type"))
    }

    fn float(node: Node) -> ParseResult<f64> {
        node.as_str().parse().into_parse_result(node.as_span())
    }

    #[inline]
    fn double_quoted_string(node: Node) -> ParseResult<String> {
        _parse_quoted_string(node, Rule::double_quoted_string_text)
    }

    #[inline]
    fn single_quoted_string(node: Node) -> ParseResult<String> {
        _parse_quoted_string(node, Rule::single_quoted_string_text)
    }

    #[inline]
    fn raw_string_start(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    #[inline]
    fn raw_string_end(_node: Node) -> ParseResult<()> {
        Ok(())
    }

    #[inline]
    fn raw_string_text(node: Node) -> ParseResult<String> {
        Ok(node.as_str().to_string())
    }

    fn raw_string(node: Node) -> ParseResult<String> {
        Ok(match_nodes! {
            node.children();
            [raw_string_start(_), raw_string_text(t), raw_string_end(_)] => t,
        })
    }

    fn esc(node: Node) -> ParseResult<Option<char>> {
        Ok(match_nodes! {
            node.children();
            [esc_alias(c)] => c,
            [esc_unicode(c)] => Some(c),
        })
    }

    fn esc_alias(node: Node) -> ParseResult<Option<char>> {
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

    fn esc_unicode(node: Node) -> ParseResult<char> {
        let code_point =
            u32::from_str_radix(node.as_str(), 16).into_parse_result(node.as_span())?;

        char::try_from(code_point).into_parse_result(node.as_span())
    }

    fn inline_sequence(node: Node) -> ParseResult<Vec<Value>> {
        let mut seq = Vec::new();

        if let Ok(values) = node.children().single() {
            for node in values.children() {
                seq.push(Parser::value(node)?);
            }
        }

        Ok(seq)
    }

    fn rhs(node: Node) -> ParseResult<AstLeaf> {
        Ok(match_nodes! {
            node.children();
            [value(v)] => AstLeaf::Value(v),
            [inline_sequence(s)] => AstLeaf::InlineSequence(s),
            [enum_variant(v)] => AstLeaf::UnitEnumVariant(v),
        })
    }

    fn enum_variant(node: Node) -> ParseResult<&str> {
        Ok(node.children().single().unwrap().as_str())
    }

    fn index(node: Node) -> ParseResult<usize> {
        node.children()
            .single()
            .unwrap()
            .as_str()
            .parse()
            .into_parse_result(node.as_span())
    }

    fn field_name(node: Node) -> ParseResult<&str> {
        Ok(node.as_str())
    }

    fn map_key(node: Node) -> ParseResult<String> {
        Ok(match_nodes! {
            node.children().single().unwrap().children();
            [single_quoted_string(k)] => k,
            [double_quoted_string(k)] => k,
        })
    }

    fn path_item(node: Node) -> ParseResult<PathItem> {
        Ok(match_nodes! {
            node.children();
            [field_name(n)] => PathItem::FieldName(n),
            [enum_variant(v)] => PathItem::EnumVariant(v),
            [map_key(k)] => PathItem::MapKey(k),
            [index(i)] => PathItem::Index(i)
        })
    }

    #[inline]
    fn path(node: Node) -> ParseResult<Node> {
        Ok(node)
    }

    fn value_assignment(node: Node) -> ParseResult<AstNode> {
        let (path, rhs) = match_nodes! {
            node.children();
            [path(path_items), rhs(rhs)] => (path_items, rhs),
        };

        let mut ast_node = AstNode::Leaf(rhs);

        for node in path.into_children().rev() {
            if node.as_rule() == Rule::path_item {
                let span = node.as_span();

                ast_node = Parser::path_item(node)?.into_ast_node(ast_node, span)?;
            }
        }

        Ok(ast_node)
    }
}

pub fn parse(input: &str) -> Result<AstNode> {
    let ast = Rc::new(RefCell::new(None));

    parse_rule(Rule::value_assignment, input, Rc::clone(&ast))
        .and_then(Parser::value_assignment)
        .map_err(Box::new)
        .map_err(ParseError)
        .map_err(Error::Parsing)?;

    let mut ast_mut = ast.borrow_mut();

    Ok(ast_mut.take().unwrap())
}

// NOTE: Parser macro confuses the compiler making it think that the function is unused, so
// we prefix it with `_` as a workaround.
#[allow(clippy::result_large_err)]
fn _parse_quoted_string(node: Node, text_rule: Rule) -> ParseResult<String> {
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

#[allow(clippy::result_large_err)]
fn parse_rule<'i>(rule: Rule, input: &'i str, ctx: Context<'i>) -> ParseResult<Node<'i>> {
    Parser::parse_with_userdata(rule, input, ctx)
        .map_err(rename_rules)
        .and_then(|p| p.single())
}

pub fn rename_rules(err: PestError<Rule>) -> PestError<Rule> {
    err.renamed_rules(|rule| {
        match rule {
            Rule::pos_int => "positive integer",
            Rule::neg_int => "negative integer",
            Rule::hex_digits => "hexadecimal digits",
            Rule::dec_digits => "digits",
            Rule::null => "`null`",
            Rule::boolean | Rule::boolean_true | Rule::boolean_false => "boolean value",
            Rule::value => "value",
            Rule::float => "floating point number",
            Rule::exponent => "exponent",
            Rule::double_quoted_string
            | Rule::double_quoted_string_content
            | Rule::double_quoted_string_text => "double quoted string",
            Rule::single_quoted_string
            | Rule::single_quoted_string_content
            | Rule::single_quoted_string_text => "single quoted string",
            Rule::esc => "escape sequence",
            Rule::esc_alias => "`\\\"`, `\\\\`, `\\/`, `\\b`, `\\f`, `\\n`, `\\r`, `\\t` or a new line",
            Rule::esc_unicode => "unicode character escape sequence",
            Rule::inline_sequence | Rule::inline_sequence_values => "inline sequence",
            Rule::rhs => "assignment right hand side",
            Rule::index | Rule::index_digits => "sequence index",
            Rule::field_name => "field name",
            Rule::enum_variant | Rule::enum_variant_ident => "enum variant",
            Rule::map_key | Rule::map_key_literal => "map key",
            Rule::path_item => "path item",
            Rule::value_assignment => "value assignment",
            Rule::path_start => "`>` followed by optional whitespace",
            Rule::path => "value path",
            Rule::path_sep => "`>` surrounded by optional whitespace or two consequtive `>`s separated by a new line",
            Rule::line_wrap_path_sep => "two consequtive `>`s separated by a new line",
            Rule::raw_string_lang_ident
            | Rule::raw_string_start => "raw string start: ``` followed by an optional language identifier, followed by a mandatory new line",
            Rule::raw_string_end => "raw string end: a new line followed by ```",
            Rule::raw_string_text => "raw string text",
            Rule::raw_string => "raw string",
            Rule::SPACE => "` ` or `\\t`",
            Rule::WHITESPACE => "whitespace",
        }.into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    macro_rules! parse {
        ($rule:ident $input:expr) => {{
            parse_rule(Rule::$rule, $input, Rc::new(RefCell::new(None))).and_then(Parser::$rule)
        }};
    }

    macro_rules! ok {
        ($rule:ident $input:expr => $expected:expr) => {
            assert_eq!(parse!($rule $input), Ok($expected));
        };
    }

    macro_rules! err {
        ($rule:ident $input:expr => $expected:expr) => {
            assert_eq!(
                parse!($rule $input).unwrap_err().to_string(),
                indoc!($expected)
            );
        };
    }

    // TODO remove
    #[allow(unused_macros)]
    macro_rules! print_err {
        ($rule:ident $input:expr) => {
            println!("{}", parse!($rule $input).unwrap_err().to_string());
        };
    }

    #[test]
    fn parse_null() {
        ok! { null "null" => () }
    }

    #[test]
    fn parse_boolean() {
        ok! { boolean "true" => true }
        ok! { boolean "false" => false }
    }

    #[test]
    fn parse_pos_int() {
        ok! { pos_int "42" => 42 }
        ok! { pos_int "0x2A" => 42 }
        ok! { pos_int "0" => 0 }
        ok! { pos_int "18446744073709551615" => 18_446_744_073_709_551_615 }
        ok! { pos_int "058" => 58 }

        err! { pos_int "18446744073709551616" =>
            " --> 1:1
            |
          1 | 18446744073709551616
            | ^------------------^
            |
            = number too large to fit in target type"
        }

        err! { pos_int "99999999999999999999999999999" =>
            " --> 1:1
            |
          1 | 99999999999999999999999999999
            | ^---------------------------^
            |
            = number too large to fit in target type"
        }

        err! { pos_int "99999999999999999999999999999" =>
            " --> 1:1
            |
          1 | 99999999999999999999999999999
            | ^---------------------------^
            |
            = number too large to fit in target type"
        }
    }

    #[test]
    fn parse_neg_int() {
        ok! { neg_int "-42" => -42 }
        ok! { neg_int "-0x2a" => -42 }
        ok! { neg_int "-9223372036854775808" => -9223372036854775808 }
        ok! { neg_int "-058" => -58 }

        err! { neg_int "-abc" =>
            " --> 1:2
            |
          1 | -abc
            |  ^---
            |
            = expected digits"
        }

        err! { neg_int "-9223372036854775809" =>
            " --> 1:1
            |
          1 | -9223372036854775809
            | ^------------------^
            |
            = number too small to fit in target type"
        }
    }

    #[test]
    fn parse_float() {
        ok! { float "42.42" => 42.42 }
        ok! { float "-42.42" => -42.42 }
        ok! { float "8." => 8.0 }
        ok! { float "1.956e2" => 1.956e2 }
        ok! { float "1.956E2" => 1.956e2 }
        ok! { float "1.956e+10" => 1.956e+10 }
        ok! { float "1.956e-10" => 1.956e-10 }
        ok! { float "-1.956e-10" => -1.956e-10 }
        ok! { float "1.7976931348623157E+308" => 1.7976931348623157E+308 }
        ok! { float "-1.7976931348623157E+308" => -1.7976931348623157E+308 }
    }

    #[test]
    fn parse_double_quoted_string() {
        ok! { double_quoted_string r#""""# => "".to_string() }
        ok! { double_quoted_string r#""foobar baz  qux""# => "foobar baz  qux".to_string() }
        ok! { double_quoted_string r#""foo \u41\u0042 bar\u00004300""# => "foo AB barC00".to_string() }

        ok! { double_quoted_string "\"foo\n\nbar\"" => "foo\n\nbar".to_string() }
        ok! { double_quoted_string "\"foo\\\nbar\"" => "foobar".to_string() }
        ok! { double_quoted_string "\"foo\\\r\nbar\"" => "foobar".to_string() }
        ok! { double_quoted_string "\"foo\\\rbar\"" => "foobar".to_string() }

        ok! {
            double_quoted_string r#""\n foo \t\r \\ baz \" bar \\n""#  =>
            "\n foo \t\r \\ baz \" bar \\n".to_string()
        }

        ok! {
            double_quoted_string r#"" \" \\ \/ \b \f \n \r \t ""# =>
            " \" \\ / \x08 \x0c \n \r \t ".to_string()
        }

        ok! {
            double_quoted_string r#"" \" \\ \/ \b \f \n \r \t ""# =>
            " \" \\ / \x08 \x0c \n \r \t ".to_string()
        }

        ok! {
            double_quoted_string r#"" foo \u2764 \u23F0 bar \u1f33a \n""# =>
            " foo ❤ ⏰ bar 🌺 \n".to_string()
        }

        err! { double_quoted_string r#"" foo \h bar ""# =>
            r#" --> 1:8
            |
          1 | " foo \h bar "
            |        ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t` or a new line"#
        };

        err! { double_quoted_string r#"" foo \u110000 bar ""# =>
            r#" --> 1:9
            |
          1 | " foo \u110000 bar "
            |         ^----^
            |
            = converted integer out of range for `char`"#
        }

        err! { double_quoted_string r#"" foo "# =>
            r#" --> 1:7
            |
          1 | " foo 
            |       ^---
            |
            = expected double quoted string or escape sequence"#
        }
    }

    #[test]
    fn parse_raw_string() {
        ok! { raw_string "```\nfoo\n\nbar\\nbaz\n```" => "foo\n\nbar\\nbaz".to_string() }
        ok! { raw_string "```\n```\n```" => "```".to_string() }
        ok! { raw_string "```\n\nabc\n\n```" => "\nabc\n".to_string() }
        ok! { raw_string "```rust   \nfoobar\n```" => "foobar".to_string() }
        ok! { raw_string "```rust   \n```rust\n```" => "```rust".to_string() }

        ok! {
            raw_string "```rust\n\t\t  f\n o \n obar  \t\t\n```" =>
            "\t\t  f\n o \n obar  \t\t".to_string()
        }

        err! { raw_string "```foo^bar\ntest\n```" =>
            " --> 1:1
            |
          1 | ```foo^bar
            | ^---
            |
            = expected raw string start: ``` followed by an optional language identifier, followed by a mandatory new line"
        }

        err! { raw_string "```\n foo ```" =>
            " --> 2:9
            |
          2 |  foo ```
            |         ^---
            |
            = expected raw string end: a new line followed by ```"
        }

        err! { raw_string "```\n foo" =>
            " --> 2:5
            |
          2 |  foo
            |     ^---
            |
            = expected raw string end: a new line followed by ```"
        }

        err! { raw_string "``` foo" =>
            " --> 1:4
            |
          1 | ``` foo
            |    ^---
            |
            = expected raw string start: ``` followed by an optional language identifier, followed by a mandatory new line"
        }

        err! { raw_string "```rust foo" =>
            " --> 1:1
            |
          1 | ```rust foo
            | ^---
            |
            = expected raw string start: ``` followed by an optional language identifier, followed by a mandatory new line"
        }
    }

    #[test]
    fn parse_single_quoted_string() {
        ok! { single_quoted_string r#"''"# => "".to_string() }
        ok! { single_quoted_string r#"'foobar baz  qux'"# => "foobar baz  qux".to_string() }
        ok! { single_quoted_string r#"'foo \u41\u0042 bar\u00004300'"# => "foo AB barC00".to_string() }

        ok! { single_quoted_string "'foo\n\nbar'" => "foo\n\nbar".to_string() }
        ok! { single_quoted_string "'foo\\\nbar'" => "foobar".to_string() }
        ok! { single_quoted_string "'foo\\\r\nbar'" => "foobar".to_string() }
        ok! { single_quoted_string "'foo\\\rbar'" => "foobar".to_string() }

        ok! {
            single_quoted_string r#"'\n foo \t\r \\ baz \" bar \\n'"#  =>
            "\n foo \t\r \\ baz \" bar \\n".to_string()
        }

        ok! {
            single_quoted_string r#"' \" \\ \/ \b \f \n \r \t '"# =>
            " \" \\ / \x08 \x0c \n \r \t ".to_string()
        }

        ok! {
            single_quoted_string r#"' \" \\ \/ \b \f \n \r \t '"# =>
            " \" \\ / \x08 \x0c \n \r \t ".to_string()
        }

        ok! {
            single_quoted_string r#"' foo \u2764 \u23F0 bar \u1f33a \n'"# =>
            " foo ❤ ⏰ bar 🌺 \n".to_string()
        }

        err! { single_quoted_string r#"' foo \h bar '"# =>
            r#" --> 1:8
            |
          1 | ' foo \h bar '
            |        ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t` or a new line"#
        };

        err! { single_quoted_string r#"' foo \u110000 bar '"# =>
            r#" --> 1:9
            |
          1 | ' foo \u110000 bar '
            |         ^----^
            |
            = converted integer out of range for `char`"#
        }

        err! { single_quoted_string r#"' foo "# =>
            r#" --> 1:7
            |
          1 | ' foo 
            |       ^---
            |
            = expected single quoted string or escape sequence"#
        }
    }

    #[test]
    fn parse_inline_sequence() {
        ok! { inline_sequence "[ ]" => vec![] }
        ok! { inline_sequence "[ \n ]" => vec![] }

        ok! {
            inline_sequence "[ 41 ,  \n 42, 43, ]" =>
            vec![Value::PosInt(41), Value::PosInt(42), Value::PosInt(43)]
        }

        ok! {
            inline_sequence "[null, true, 42, -42, 42.42, \"foo bar\", 'baz qux']" =>
            vec![
                Value::Null,
                Value::Bool(true),
                Value::PosInt(42),
                Value::NegInt(-42),
                Value::Float(42.42),
                Value::String("foo bar".into()),
                Value::String("baz qux".into())
            ]
        }

        err! { inline_sequence "[ , ]" =>
            " --> 1:3
            |
          1 | [ , ]
            |   ^---
            |
            = expected value"
        }

        err! { inline_sequence "[ true, nottrue ]" =>
            " --> 1:9
            |
          1 | [ true, nottrue ]
            |         ^---
            |
            = expected value"
        }
    }

    #[test]
    fn parse_enum_variant() {
        ok! { enum_variant "`Foo`" => "Foo" }
        ok! { enum_variant "`Foo_Bar_123_baz`" => "Foo_Bar_123_baz" }

        err! { enum_variant "`Foo" =>
            " --> 1:1
            |
          1 | `Foo
            | ^---
            |
            = expected enum variant"
        }

        err! { enum_variant "`Foo Bar`" =>
            " --> 1:1
            |
          1 | `Foo Bar`
            | ^---
            |
            = expected enum variant"
        }

        err! { enum_variant "` foo`" =>
            " --> 1:2
            |
          1 | ` foo`
            |  ^---
            |
            = expected enum variant"
        }
    }

    #[test]
    fn parse_rhs() {
        ok! { rhs "`Foo`" => AstLeaf::UnitEnumVariant("Foo") }

        ok! {
            rhs "[1, 2]" =>
            AstLeaf::InlineSequence(vec![Value::PosInt(1), Value::PosInt(2)])
        }

        ok! { rhs "null" => AstLeaf::Value(Value::Null) }

        ok! { rhs "true" => AstLeaf::Value(Value::Bool(true)) }
        ok! { rhs "false" => AstLeaf::Value(Value::Bool(false)) }

        ok! { rhs "42" => AstLeaf::Value(Value::PosInt(42)) }
        ok! { rhs "0x2A" => AstLeaf::Value(Value::PosInt(42)) }

        ok! { rhs "-42" => AstLeaf::Value(Value::NegInt(-42)) }
        ok! { rhs "-0x2A" => AstLeaf::Value(Value::NegInt(-42)) }

        ok! { rhs "42." => AstLeaf::Value(Value::Float(42.0)) }
        ok! { rhs "42.42" => AstLeaf::Value(Value::Float(42.42)) }
        ok! { rhs "-42.42" => AstLeaf::Value(Value::Float(-42.42)) }
        ok! { rhs "1.956e-10" => AstLeaf::Value(Value::Float(1.956e-10)) }

        ok! { rhs "\" foo bar \"" => AstLeaf::Value(Value::String(" foo bar ".into())) }
        ok! { rhs "' foo bar '" => AstLeaf::Value(Value::String(" foo bar ".into())) }
        ok! { rhs "```rust\n foo\nbar \n```" => AstLeaf::Value(Value::String(" foo\nbar ".into())) }
    }

    #[test]
    fn parse_index() {
        ok! { index "[0]" => 0 }
        ok! { index "[123]" => 123 }
        ok! { index "[  123  ]" => 123 }
        ok! { index "[999999999]" => 999999999 }

        err! { index "[003]" =>
            " --> 1:1
            |
          1 | [003]
            | ^---
            |
            = expected sequence index"
        }

        err! { index "[123" =>
            " --> 1:1
            |
          1 | [123
            | ^---
            |
            = expected sequence index"
        }

        err! { index "[999999999999999999999999999999999]" =>
            " --> 1:1
            |
          1 | [999999999999999999999999999999999]
            | ^---------------------------------^
            |
            = number too large to fit in target type"
        }
    }

    #[test]
    fn parse_field_name() {
        ok! { field_name "Foo" => "Foo" }
        ok! { field_name "Foo_Bar_123_baz" => "Foo_Bar_123_baz" }

        err! { field_name "123foo" =>
            " --> 1:1
            |
          1 | 123foo
            | ^---
            |
            = expected field name"
        }
    }

    #[test]
    fn parse_map_key() {
        ok! { map_key r#"["foobar baz  qux"]"# => "foobar baz  qux".to_string() }
        ok! { map_key r#"[  "foo \u41\u0042 bar\u00004300"  ]"# => "foo AB barC00".to_string() }

        ok! { map_key r#"['foobar baz  qux']"# => "foobar baz  qux".to_string() }
        ok! { map_key r#"[  'foo \u41\u0042 bar\u00004300'  ]"# => "foo AB barC00".to_string() }

        err! { map_key "[foobar]" =>
            " --> 1:2
            |
          1 | [foobar]
            |  ^---
            |
            = expected map key"
        }
    }

    #[test]
    fn path_item() {
        ok! { path_item "foo_bar" => PathItem::FieldName("foo_bar") }
        ok! { path_item "`FooBar`" => PathItem::EnumVariant("FooBar") }
        ok! { path_item "[42]" => PathItem::Index(42) }
        ok! { path_item "[\"foobar\"]" => PathItem::MapKey("foobar".into()) }
        ok! { path_item "['foobar']" => PathItem::MapKey("foobar".into()) }
    }

    #[test]
    fn value_assignment() {
        ok! {
            value_assignment "> foo > bar   > baz = 42" =>
            AstNode::Fields(map!(
                "foo" => AstNode::Fields(map!(
                    "bar" => AstNode::Fields(map!(
                        "baz" => AstNode::Leaf(AstLeaf::Value(Value::PosInt(42)))
                    ))
                ))
            ))
        };

        ok! {
            value_assignment "> foo_bar > [0] > `Baz` > ['qux quz'] = [1, 2, 3]" =>
            AstNode::Fields(map!(
                "foo_bar" => AstNode::Sequence(vec![
                    AstNode::NewTypeEnumVariant("Baz", AstNode::Map(map!(
                        "qux quz".into() => AstNode::Leaf(AstLeaf::InlineSequence(vec![
                            Value::PosInt(1),
                            Value::PosInt(2),
                            Value::PosInt(3),
                        ]))
                    )).into())
                ])
            ))
        }

        ok! {
            value_assignment "> = `Hello`" =>
            AstNode::Leaf(AstLeaf::UnitEnumVariant("Hello"))
        };

        ok! {
            value_assignment "> foo_bar = `Hello`" =>
            AstNode::Fields(map!("foo_bar" =>
                AstNode::Leaf(AstLeaf::UnitEnumVariant("Hello"))
            ))
        }

        ok! {
            value_assignment "> `Hello` >    \n> `World` = true" =>
            AstNode::NewTypeEnumVariant(
                "Hello",
                AstNode::NewTypeEnumVariant(
                    "World",
                    AstNode::Leaf(AstLeaf::Value(Value::Bool(true))).into(),
                )
                .into(),
            )
        }

        ok! {
            value_assignment "> ['>'] = `Hello`" =>
            AstNode::Map(map!(">".into() =>
                AstNode::Leaf(AstLeaf::UnitEnumVariant("Hello"))
            ))
        }
    }
}
