use konfig_edit::error::{Error, Result};

pub(super) trait Float: ryu::Float {
    fn is_finite(self) -> bool;
}

impl Float for f32 {
    #[inline]
    fn is_finite(self) -> bool {
        f32::is_finite(self)
    }
}

impl Float for f64 {
    #[inline]
    fn is_finite(self) -> bool {
        f64::is_finite(self)
    }
}

pub(super) fn write_escaped_str(out: &mut String, v: &str) {
    let mut start = 0;

    out.push('"');

    for (i, c) in v.char_indices() {
        if let Some(esc) = escape_char(c) {
            if start < i {
                out.push_str(&v[start..i]);
            }

            out.push_str(esc);
            start = i + 1;
        }
    }

    if start < v.len() {
        out.push_str(&v[start..]);
    }

    out.push('"');
}

#[inline]
pub(crate) fn write_int(out: &mut String, v: impl itoa::Integer) {
    let mut buffer = itoa::Buffer::new();

    out.push_str(buffer.format(v));
}

pub(super) fn write_float(out: &mut String, v: impl Float) -> Result<()> {
    if !v.is_finite() {
        return Err(Error::InfAndNanNotSupported);
    }

    let mut buffer = ryu::Buffer::new();

    out.push_str(buffer.format_finite(v));

    Ok(())
}

pub(super) fn make_map_key(
    key_serializer: impl FnOnce(&mut String) -> Result<()>,
) -> Result<String> {
    let mut key = String::with_capacity(16);

    key.push('[');
    key_serializer(&mut key)?;
    key.push(']');

    Ok(key)
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
