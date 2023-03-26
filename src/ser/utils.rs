pub(crate) fn escape_char(c: char) -> Option<&'static str> {
    const UNICODE_ESCAPES: &[&str] = &[
        "\\u0000", "\\u0001", "\\u0002", "\\u0003", "\\u0004", "\\u0005", "\\u0006", "\\u0007",
        "\\u0008", "\\u0009", "\\u000a", "\\u000b", "\\u000c", "\\u000d", "\\u000e", "\\u000f",
        "\\u0010", "\\u0011", "\\u0012", "\\u0013", "\\u0014", "\\u0015", "\\u0016", "\\u0017",
        "\\u0018", "\\u0019", "\\u001a", "\\u001b", "\\u001c", "\\u001d", "\\u001e", "\\u001f",
    ];

    match c {
        '\x08' => Some(r#"\b"#),
        '\t' => Some(r#"\t"#),
        '\n' => Some(r#"\n"#),
        '\x0C' => Some(r#"\f"#),
        '\r' => Some(r#"\r"#),
        '"' => Some(r#"\""#),
        '\\' => Some(r#"\\"#),
        '\x00'..='\x1F' => Some(UNICODE_ESCAPES[c as usize]),
        _ => None,
    }
}

#[inline]
pub(crate) fn write_int(out: &mut String, v: impl itoa::Integer) {
    let mut buffer = itoa::Buffer::new();

    out.push_str(buffer.format(v));
}

#[inline]
pub(crate) fn write_float(out: &mut String, v: impl ryu::Float) {
    let mut buffer = ryu::Buffer::new();

    out.push_str(buffer.format(v));
}
