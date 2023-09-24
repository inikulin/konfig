use super::formatting::DocLineEscape;
use crate::error::{Error, Result};
use std::borrow::Cow;
use std::fmt::{self, Write};

pub fn write_int(out: &mut impl Write, v: impl itoa::Integer) -> fmt::Result {
    let mut buffer = itoa::Buffer::new();

    out.write_str(buffer.format(v))
}

pub fn write_float(out: &mut impl Write, v: f64) -> Result<()> {
    if !v.is_finite() {
        return Err(Error::InfAndNanNotSupported);
    }

    let mut buffer = ryu::Buffer::new();

    out.write_str(buffer.format_finite(v))
        .map_err(Error::custom)?;

    Ok(())
}

pub fn write_escaped_str(out: &mut impl Write, v: &str) -> fmt::Result {
    let mut start = 0;

    for (i, c) in v.char_indices() {
        if let Some(esc) = escape_char(c) {
            if start < i {
                out.write_str(&v[start..i])?;
            }

            out.write_str(esc)?;
            start = i + 1;
        }
    }

    if start < v.len() {
        out.write_str(&v[start..])?;
    }

    Ok(())
}

pub fn escape_docs<'d>(docs: &'d str, escape: &dyn DocLineEscape) -> Cow<'d, str> {
    let mut out = Cow::Borrowed(docs);

    'line_iter: for (line_idx, line) in docs.lines().enumerate() {
        for (idx, ch) in line.chars().enumerate() {
            if ch == ' ' || ch == '\t' {
                continue;
            }

            if ch == '>' {
                if matches!(out, Cow::Borrowed(_)) {
                    out = docs
                        .lines()
                        .take(line_idx)
                        .fold(String::new(), |a, b| a + b + "\n")
                        .into();
                }

                let mut line = line.to_owned();
                let out = out.to_mut();

                escape.escape(&mut line, idx);
                out.push_str(&line);
                out.push('\n');

                continue 'line_iter;
            }

            break;
        }

        if let Cow::Owned(ref mut out) = out {
            out.push_str(line);
            out.push('\n');
        }
    }

    out
}

fn escape_char(c: char) -> Option<&'static str> {
    const UNICODE_ESCAPES: &[&str] = &[
        "\\u000000",
        "\\u000001",
        "\\u000002",
        "\\u000003",
        "\\u000004",
        "\\u000005",
        "\\u000006",
        "\\u000007",
        "\\u000008",
        "\\u000009",
        "\\u00000a",
        "\\u00000b",
        "\\u00000c",
        "\\u00000d",
        "\\u00000e",
        "\\u00000f",
        "\\u000010",
        "\\u000011",
        "\\u000012",
        "\\u000013",
        "\\u000014",
        "\\u000015",
        "\\u000016",
        "\\u000017",
        "\\u000018",
        "\\u000019",
        "\\u00001a",
        "\\u00001b",
        "\\u00001c",
        "\\u00001d",
        "\\u00001e",
        "\\u00001f",
    ];

    match c {
        '\x08' => Some(r"\b"),
        '\t' => Some(r"\t"),
        '\n' => Some(r"\n"),
        '\x0C' => Some(r"\f"),
        '\r' => Some(r"\r"),
        '"' => Some(r#"\""#),
        '\\' => Some(r"\\"),
        '\x00'..='\x1F' => Some(UNICODE_ESCAPES[c as usize]),
        _ => None,
    }
}
