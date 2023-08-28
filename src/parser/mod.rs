pub(crate) mod error;
mod imp;
mod insertion_point;
mod path_item;

use self::error::{rename_rules, ParseError, ParseResult};
use self::imp::{Node, Parser, Rule};
use crate::error::{Error, Result};
use crate::value::ValueCell;
use pest::Span;
use pest_consume::Parser as _;
use std::cell::RefCell;
use std::rc::Rc;

type Ast = Rc<RefCell<Option<ValueCell>>>;

pub fn parse(input: &str) -> Result<ValueCell> {
    let ast = Rc::new(RefCell::new(None));

    parse_rule(Rule::konfig, input, Rc::clone(&ast))
        .and_then(Parser::konfig)
        .map_err(Box::new)
        .map_err(ParseError)
        .map_err(Error::Parsing)?;

    let mut ast_mut = ast.borrow_mut();

    Ok(ast_mut.take().unwrap())
}

#[allow(clippy::result_large_err)]
fn parse_rule(rule: Rule, input: &str, ast: Ast) -> ParseResult<Node> {
    #[cfg(any(test, feature = "test_assertions"))]
    crate::value::value_cell_safety_checks::IS_PARSING.with(|is_parsing| is_parsing.set(true));

    let res = Parser::parse_with_userdata(rule, input, ast)
        .map_err(rename_rules)
        .and_then(|p| p.single());

    #[cfg(any(test, feature = "test_assertions"))]
    crate::value::value_cell_safety_checks::IS_PARSING.with(|is_parsing| is_parsing.set(false));

    res
}

#[cfg(test)]
mod tests {
    use super::path_item::PathItem;
    use super::*;
    use crate::value::{Primitive, Value};
    use indoc::indoc;

    macro_rules! parse {
        ($rule:ident $input:expr) => {{
            parse_rule(Rule::$rule, $input, Rc::new(RefCell::new(None))).and_then(Parser::$rule)
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
        ok! { double_quoted_string r#""foo \u41\u0042 bar\u00004300""# => "foo AB barC00".to_string() }

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
        ok! { single_quoted_string r#"'foo \u41\u0042 bar\u00004300'"# => "foo AB barC00".to_string() }

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
    fn parse_sequence_of_primitives() {
        ok! { sequence_of_primitives "[ ]" => vec![] }
        ok! { sequence_of_primitives "[ \n ]" => vec![] }

        ok! {
            sequence_of_primitives "[ 41 ,  \n 42, 43, ]" =>
            vec![Primitive::PosInt(41), Primitive::PosInt(42), Primitive::PosInt(43)]
        }

        ok! {
            sequence_of_primitives "[null, true, 42, -42, 42.42, \"foo bar\", 'baz qux']" =>
            vec![
                Primitive::Null,
                Primitive::Bool(true),
                Primitive::PosInt(42),
                Primitive::NegInt(-42),
                Primitive::Float(42.42),
                Primitive::String("foo bar".into()),
                Primitive::String("baz qux".into())
            ]
        }

        err! { sequence_of_primitives "[ , ]" =>
            " --> 1:3
            |
          1 | [ , ]
            |   ^---
            |
            = expected primitive value"
        }

        err! { sequence_of_primitives "[ true, nottrue ]" =>
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
        ok! { rhs "`Foo`" => Value::Primitive(Primitive::UnitVariant("Foo".into())) }

        ok! {
            rhs "[1, 2]" =>
            Value::SequenceOfPrimitives(vec![Primitive::PosInt(1), Primitive::PosInt(2)])
        }

        ok! { rhs "null" => Value::Primitive(Primitive::Null) }

        ok! { rhs "true" => Value::Primitive(Primitive::Bool(true)) }
        ok! { rhs "false" => Value::Primitive(Primitive::Bool(false)) }

        ok! { rhs "42" => Value::Primitive(Primitive::PosInt(42)) }
        ok! { rhs "0x2A" => Value::Primitive(Primitive::PosInt(42)) }

        ok! { rhs "-42" => Value::Primitive(Primitive::NegInt(-42)) }
        ok! { rhs "-0x2A" => Value::Primitive(Primitive::NegInt(-42)) }

        ok! { rhs "42." => Value::Primitive(Primitive::Float(42.0)) }
        ok! { rhs "42.42" => Value::Primitive(Primitive::Float(42.42)) }
        ok! { rhs "-42.42" => Value::Primitive(Primitive::Float(-42.42)) }
        ok! { rhs "1.956e-10" => Value::Primitive(Primitive::Float(1.956e-10)) }

        ok! { rhs "\" foo bar \"" => Value::Primitive(Primitive::String(" foo bar ".into())) }
        ok! { rhs "' foo bar '" => Value::Primitive(Primitive::String(" foo bar ".into())) }
        ok! { rhs "\n```rust\n foo\nbar \n```" => Value::Primitive(Primitive::String(" foo\nbar ".into())) }
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
    fn parse_path_item() {
        ok! { path_item "foo_bar" => PathItem::FieldName("foo_bar") }
        ok! { path_item "`FooBar`" => PathItem::EnumVariant("FooBar") }
        ok! { path_item "[42]" => PathItem::Index(42) }
        ok! { path_item "[\"foobar\"]" => PathItem::MapKey("foobar".into()) }
        ok! { path_item "['foobar']" => PathItem::MapKey("foobar".into()) }
    }
}
