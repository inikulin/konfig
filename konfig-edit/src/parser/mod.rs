pub(crate) mod error;
mod imp;
mod insertion_point;

use self::error::{parse_error, rename_rules, ParseError, ParseResult};
use self::imp::{Node, Parser, Rule};
use crate::error::Result;
use crate::value::ValueCell;
use pest::Span;
use pest_consume::Parser as _;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
struct Context {
    root: Option<ValueCell>,
    last_rhs: Option<ValueCell>,
    pending_docs: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct LexicalInfo {
    pub is_rhs_seq: bool,
    pub docs_before: String,
    pub docs_after: String,
}

pub fn parse(input: &str) -> Result<ValueCell> {
    #[cfg(debug_assertions)]
    let _guard = crate::value::value_cell::safety_checks::ParsingGuard::new();

    let ctx = Default::default();

    parse_rule(Rule::konfig, input, Rc::clone(&ctx))
        .and_then(Parser::konfig)
        .map_err(ParseError::wrap)?;

    let mut ctx = ctx.borrow_mut();

    let Some(last_rhs) = ctx.last_rhs.take() else {
        let end = input.len().saturating_sub(1);

        return Err(ParseError::wrap(parse_error!(
            Span::new(input, end, end).unwrap(),
            "konfig should contain some expressions"
        )));
    };

    if let Some(docs) = ctx.pending_docs.take() {
        last_rhs.borrow_mut().lexical_info.docs_after = docs;
    }

    Ok(ctx.root.take().unwrap())
}

#[allow(clippy::result_large_err)]
fn parse_rule(rule: Rule, input: &str, context: Rc<RefCell<Context>>) -> ParseResult<Node> {
    Parser::parse_with_userdata(rule, input, context)
        .map_err(rename_rules)
        .and_then(|p| p.single())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{PathItem, Value};
    use indoc::indoc;

    macro_rules! parse {
        ($rule:ident $input:expr) => {{
            #[cfg(debug_assertions)]
            let _guard = crate::value::value_cell::safety_checks::ParsingGuard::new();

            parse_rule(Rule::$rule, $input, Default::default()).and_then(Parser::$rule)
        }};
    }

    macro_rules! ok {
        ($rule:ident $input:expr => $expected:expr) => {
            let parsed = match parse!($rule $input) {
                Ok(parsed) => parsed,
                Err(err) => panic!("\n{}", err)
            };

            assert_eq!(parsed, $expected);
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
        ok! { double_quoted_string r#""foo \u000041\u000042 bar\u00004300""# => "foo AB barC00".to_string() }

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
            double_quoted_string r#"" foo \u002764 \u0023F0 bar \u01f33a \n""# =>
            " foo ❤ ⏰ bar 🌺 \n".to_string()
        }

        err! { double_quoted_string "\"foo\\\nbar\"" =>
            r#" --> 1:6
            |
          1 | "foo\␊
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
        }

        err! { double_quoted_string "\"foo\\\r\nbar\"" =>
            r#" --> 1:6
            |
          1 | "foo\␍␊
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
        }

        err! { double_quoted_string "\"foo\\\rbar\"" =>
            r#" --> 1:6
            |
          1 | "foo\␍bar"
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"# 
        }

        err! { double_quoted_string "\"foo\n\nbar\"" =>
            r#" --> 1:5
            |
          1 | "foo␊
            |     ^---
            |
            = expected double quoted string or escape sequence"#
        }

        err! { double_quoted_string r#"" foo \h bar ""# =>
            r#" --> 1:8
            |
          1 | " foo \h bar "
            |        ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
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
        ok! { raw_string "\n```\nfoo\n\nbar\\nbaz\n```" => "foo\n\nbar\\nbaz".to_string() }
        ok! { raw_string "\n```\n```\n```" => "```".to_string() }
        ok! { raw_string "\n```\n\nabc\n\n```" => "\nabc\n".to_string() }
        ok! { raw_string "\n```rust   \nfoobar\n```" => "foobar".to_string() }
        ok! { raw_string "\n```rust   \n```rust\n```" => "```rust".to_string() }

        ok! {
            raw_string "\n```rust\n\t\t  f\n o \n obar  \t\t\n```" =>
            "\t\t  f\n o \n obar  \t\t".to_string()
        }

        err! { raw_string "\n```foo^bar\ntest\n```" =>
            " --> 1:1
            |
          1 | ␊
            | ^---
            |
            = expected raw string start: new line, followed by ```, followed by an optional language identifier, followed by a mandatory new line"
        }

        err! { raw_string "\n```\n foo ```" =>
            " --> 3:9
            |
          3 |  foo ```
            |         ^---
            |
            = expected raw string end: a new line followed by ```"
        }

        err! { raw_string "\n```\n foo" =>
            " --> 3:5
            |
          3 |  foo
            |     ^---
            |
            = expected raw string end: a new line followed by ```"
        }

        err! { raw_string "\n``` foo" =>
            " --> 1:1
            |
          1 | ␊
            | ^---
            |
            = expected raw string start: new line, followed by ```, followed by an optional language identifier, followed by a mandatory new line"
        }

        err! { raw_string "\n```rust foo" =>
            " --> 1:1
            |
          1 | ␊
            | ^---
            |
            = expected raw string start: new line, followed by ```, followed by an optional language identifier, followed by a mandatory new line"
        }

        err! { raw_string "```\n foo\n```" =>
            " --> 1:1
            |
          1 | ```
            | ^---
            |
            = expected raw string start: new line, followed by ```, followed by an optional language identifier, followed by a mandatory new line"
        }
    }

    #[test]
    fn parse_single_quoted_string() {
        ok! { single_quoted_string r#"''"# => "".to_string() }
        ok! { single_quoted_string r#"'foobar baz  qux'"# => "foobar baz  qux".to_string() }
        ok! { single_quoted_string r#"'foo \u000041\u000042 bar\u00004300'"# => "foo AB barC00".to_string() }

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
            single_quoted_string r#"' foo \u002764 \u0023F0 bar \u01f33a \n'"# =>
            " foo ❤ ⏰ bar 🌺 \n".to_string()
        }

        err! { single_quoted_string "'foo\\\nbar'" =>
            r#" --> 1:6
            |
          1 | 'foo\␊
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
        }

        err! { single_quoted_string "'foo\\\r\nbar'" =>
            r#" --> 1:6
            |
          1 | 'foo\␍␊
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
        }

        err! { single_quoted_string "'foo\\\rbar'" =>
            r#" --> 1:6
            |
          1 | 'foo\␍bar'
            |      ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"# 
        }

        err! { single_quoted_string "'foo\n\nbar'" =>
            r#" --> 1:5
            |
          1 | 'foo␊
            |     ^---
            |
            = expected single quoted string or escape sequence"#
        }

        err! { single_quoted_string r#"' foo \h bar '"# =>
            r#" --> 1:8
            |
          1 | ' foo \h bar '
            |        ^---
            |
            = expected `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`"#
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
    fn parse_list_of_primitives() {
        ok! {
            list_of_primitives "\n> -1\n> - '2'\n>- `three`\n   >--4.0" =>
            Value::Sequence(vec![
                Value::UInt(1).into(),
                Value::String("2".into()).into(),
                Value::UnitVariant("three".into()).into(),
                Value::Float(-4.0).into()
            ])
        }
    }

    #[test]
    fn parse_array_of_primitives() {
        ok! { array_of_primitives "[ ]" => Value::Sequence(vec![]) }
        ok! { array_of_primitives "[ \n ]" => Value::Sequence(vec![]) }

        ok! {
            array_of_primitives "[ 41 ,  \n 42, 43, ]" =>
            Value::Sequence(vec![
                Value::UInt(41).into(),
                Value::UInt(42).into(),
                Value::UInt(43).into()
            ])
        }

        ok! {
            array_of_primitives "[null, true, 42, -42, 42.42, \"foo bar\", 'baz qux']" =>
            Value::Sequence(vec![
                Value::Null.into(),
                Value::Bool(true).into(),
                Value::UInt(42).into(),
                Value::Int(-42).into(),
                Value::Float(42.42).into(),
                Value::String("foo bar".into()).into(),
                Value::String("baz qux".into()).into()
            ])
        }

        err! { array_of_primitives "[ , ]" =>
            " --> 1:3
            |
          1 | [ , ]
            |   ^---
            |
            = expected primitive value"
        }

        err! { array_of_primitives "[ true, nottrue ]" =>
            " --> 1:9
            |
          1 | [ true, nottrue ]
            |         ^---
            |
            = expected primitive value"
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
        ok! { rhs "`Foo`" => Value::UnitVariant("Foo".into()) }

        ok! {
            rhs "[1, 2]" =>
            Value::Sequence(vec![
               Value::UInt(1).into(),
               Value::UInt(2).into()
            ])
        }

        ok! { rhs "null" => Value::Null }

        ok! { rhs "true" => Value::Bool(true) }
        ok! { rhs "false" => Value::Bool(false) }

        ok! { rhs "42" => Value::UInt(42) }
        ok! { rhs "0x2A" => Value::UInt(42) }

        ok! { rhs "-42" => Value::Int(-42) }
        ok! { rhs "-0x2A" => Value::Int(-42) }

        ok! { rhs "42." => Value::Float(42.0) }
        ok! { rhs "42.42" => Value::Float(42.42) }
        ok! { rhs "-42.42" => Value::Float(-42.42) }
        ok! { rhs "1.956e-10" => Value::Float(1.956e-10) }

        ok! { rhs "\" foo bar \"" => Value::String(" foo bar ".into()) }
        ok! { rhs "' foo bar '" => Value::String(" foo bar ".into()) }
        ok! { rhs "\n```rust\n foo\nbar \n```" => Value::String(" foo\nbar ".into()) }
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
        ok! { map_key r#"[  "foo \u000041\u000042 bar\u00004300"  ]"# => "foo AB barC00".to_string() }

        ok! { map_key r#"['foobar baz  qux']"# => "foobar baz  qux".to_string() }
        ok! { map_key r#"[  'foo \u000041\u000042 bar\u00004300'  ]"# => "foo AB barC00".to_string() }

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
    fn parse_path_item() {
        ok! { path_item "foo_bar" => PathItem::StructFieldName("foo_bar".into()) }
        ok! { path_item "`FooBar`" => PathItem::VariantName("FooBar".into()) }
        ok! { path_item "[42]" => PathItem::SequenceIndex(42) }
        ok! { path_item "[\"foobar\"]" => PathItem::MapKey("foobar".into()) }
        ok! { path_item "['foobar']" => PathItem::MapKey("foobar".into()) }
    }
}
