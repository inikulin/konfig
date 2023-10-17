use super::imp::Rule;
use pest::Span;
use pest_consume::Error as PestError;
use std::fmt;

pub(super) type ParseResult<T> = std::result::Result<T, PestError<Rule>>;

// NOTE: a wrapper type to not expose `Rule` in the public API.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseError(Box<PestError<Rule>>);

impl ParseError {
    pub(super) fn wrap(err: PestError<Rule>) -> crate::error::Error {
        crate::error::Error::Parsing(ParseError(Box::new(err)))
    }
}

impl fmt::Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub(super) trait IntoParseResult<T> {
    #[allow(clippy::result_large_err)]
    fn into_parse_result(self, span: Span) -> ParseResult<T>;
}

impl<T, E> IntoParseResult<T> for std::result::Result<T, E>
where
    E: ToString,
{
    #[inline]
    fn into_parse_result(self, span: Span) -> ParseResult<T> {
        self.map_err(|e| parse_error!(span, "{}", e.to_string()))
    }
}

pub(super) fn rename_rules(err: PestError<Rule>) -> PestError<Rule> {
    err.renamed_rules(|rule| {
        match rule {
            Rule::pos_int => "positive integer",
            Rule::neg_int => "negative integer",
            Rule::hex_digits => "hexadecimal digits",
            Rule::dec_digits => "digits",
            Rule::null => "`null`",
            Rule::boolean => "boolean value",
            Rule::primitive => "primitive value",
            Rule::float => "floating point number",
            Rule::exponent => "exponent",
            Rule::double_quoted_string
            | Rule::double_quoted_string_content
            | Rule::double_quoted_string_text => "double quoted string",
            Rule::single_quoted_string
            | Rule::single_quoted_string_content
            | Rule::single_quoted_string_text => "single quoted string",
            Rule::esc => "escape sequence",
            Rule::esc_alias => "`\\\"`, `\\\\`, `\\/`, `\\b`, `\\f`, `\\n`, `\\r`, `\\t`",
            Rule::esc_unicode => "unicode character escape sequence",
            Rule::array_of_primitives
            | Rule::array_of_primitives_values
            | Rule::list_of_primitives => "sequence of primitive values",
            Rule::rhs => "assignment right hand side",
            Rule::index | Rule::index_digits => "sequence index",
            Rule::field_name => "field name",
            Rule::enum_variant | Rule::enum_variant_ident => "enum variant",
            Rule::map_key | Rule::map_key_literal => "map key",
            Rule::path_item => "path item",
            Rule::expr => "expression",
            Rule::path => "value path",
            Rule::raw_string_lang_ident
            | Rule::raw_string_start => "raw string start: new line, followed by ```, followed by an optional language identifier, followed by a mandatory new line",
            Rule::raw_string_end => "raw string end: a new line followed by ```",
            Rule::raw_string_text => "raw string text",
            Rule::raw_string => "raw string",
            Rule::path_start => "`>` followed by optional spaces",
            Rule::separator => "`>` followed by optional spaces or two consequtive `>` separated by a new line",
            Rule::SPACE => "` ` or `\\t`",
            Rule::INDENTATION => "optional spaces with a single optional new line",
            Rule::EOI => "end of input",
            Rule::konfig => "expression",
            Rule::docs => "documentation",
            Rule::expr_terminator => "double new line or end of input",
            Rule::docs_terminator => "new line or end of input"
        }.into()
    })
}

macro_rules! parse_error {
    ($span:expr, $msg:literal) => {
        parse_error!($span, $msg,)
    };

    ($span:expr, $msg:literal, $($arg:expr),*) => {
        pest_consume::Error::new_from_span(pest::error::ErrorVariant::CustomError {
            message: format!($msg, $($arg),*),
        }, $span.clone())
    }
}

pub(super) use parse_error;
