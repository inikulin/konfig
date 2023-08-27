pub mod ast;
mod imp;
mod insertion_point;
mod type_name;

use self::imp::{Node, ParseResult, Parser, Rule};
use crate::error::{Error, Result};
use pest::Span;
use pest_consume::{Error as PestError, Parser as PestParser};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Ast = Rc<RefCell<Option<ast::NodeCell>>>;

macro_rules! error {
    ($span:expr, $msg:literal) => {
        error!($span, $msg,)
    };
    ($span:expr, $msg:literal, $($arg:expr),*) => {
        pest_consume::Error::new_from_span(pest::error::ErrorVariant::CustomError {
            message: format!($msg, $($arg),*),
        }, $span.clone())
    }
}

use error;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseError(Box<PestError<Rule>>);

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
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
    fn into_ast_node(self, prev: ast::NodeCell, span: Span) -> ParseResult<ast::NodeCell> {
        match self {
            PathItem::Index(0) => Ok(ast::Node::Sequence(vec![prev])),
            PathItem::Index(_) => Err(error!(
                span,
                "sequence items should be defined in order, with the first item having index `0`"
            )),
            PathItem::MapKey(key) => Ok(ast::Node::Map([(key, prev)].into_iter().collect())),
            PathItem::FieldName(name) => Ok(ast::Node::Fields(
                [(name.to_string(), prev)].into_iter().collect(),
            )),
            PathItem::EnumVariant(variant) => {
                Ok(ast::Node::NewTypeEnumVariant(variant.to_string(), prev))
            }
        }
        .map(Into::into)
    }
}

pub fn parse(input: &str) -> Result<ast::NodeCell> {
    let ast = Rc::new(RefCell::new(None));

    parse_rule(Rule::value_assignment, input, Rc::clone(&ast))
        .and_then(Parser::value_assignment)
        .map_err(Box::new)
        .map_err(ParseError)
        .map_err(Error::Parsing)?;

    let mut ast_mut = ast.borrow_mut();

    Ok(ast_mut.take().unwrap())
}

#[allow(clippy::result_large_err)]
fn parse_rule(rule: Rule, input: &str, ast: Ast) -> ParseResult<Node> {
    Parser::parse_with_userdata(rule, input, ast)
        .map_err(rename_rules)
        .and_then(|p| p.single())
}

fn rename_rules(err: PestError<Rule>) -> PestError<Rule> {
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
            vec![ast::Value::PosInt(41), ast::Value::PosInt(42), ast::Value::PosInt(43)]
        }

        ok! {
            inline_sequence "[null, true, 42, -42, 42.42, \"foo bar\", 'baz qux']" =>
            vec![
                ast::Value::Null,
                ast::Value::Bool(true),
                ast::Value::PosInt(42),
                ast::Value::NegInt(-42),
                ast::Value::Float(42.42),
                ast::Value::String("foo bar".into()),
                ast::Value::String("baz qux".into())
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
        ok! { rhs "`Foo`" => ast::Leaf::UnitEnumVariant("Foo".into()) }

        ok! {
            rhs "[1, 2]" =>
            ast::Leaf::InlineSequence(vec![ast::Value::PosInt(1), ast::Value::PosInt(2)])
        }

        ok! { rhs "null" => ast::Leaf::Value(ast::Value::Null) }

        ok! { rhs "true" => ast::Leaf::Value(ast::Value::Bool(true)) }
        ok! { rhs "false" => ast::Leaf::Value(ast::Value::Bool(false)) }

        ok! { rhs "42" => ast::Leaf::Value(ast::Value::PosInt(42)) }
        ok! { rhs "0x2A" => ast::Leaf::Value(ast::Value::PosInt(42)) }

        ok! { rhs "-42" => ast::Leaf::Value(ast::Value::NegInt(-42)) }
        ok! { rhs "-0x2A" => ast::Leaf::Value(ast::Value::NegInt(-42)) }

        ok! { rhs "42." => ast::Leaf::Value(ast::Value::Float(42.0)) }
        ok! { rhs "42.42" => ast::Leaf::Value(ast::Value::Float(42.42)) }
        ok! { rhs "-42.42" => ast::Leaf::Value(ast::Value::Float(-42.42)) }
        ok! { rhs "1.956e-10" => ast::Leaf::Value(ast::Value::Float(1.956e-10)) }

        ok! { rhs "\" foo bar \"" => ast::Leaf::Value(ast::Value::String(" foo bar ".into())) }
        ok! { rhs "' foo bar '" => ast::Leaf::Value(ast::Value::String(" foo bar ".into())) }
        ok! { rhs "```rust\n foo\nbar \n```" => ast::Leaf::Value(ast::Value::String(" foo\nbar ".into())) }
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
}
