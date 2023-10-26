use konfig_edit::serializer::components::doc_line_leading_gt_sign_pos;
use konfig_edit::serializer::formatting::{DocLineEscape, MarkdowDocLineEscape};
use konfig_edit::value::{Path, PathItem};
use std::collections::HashMap;

pub type DocsWrittenFlag = bool;

pub struct DocWriter {
    docs: HashMap<Vec<PathItem<'static>>, String>,
}

impl DocWriter {
    pub fn new(docs: HashMap<Vec<PathItem<'static>>, String>) -> Self {
        Self { docs }
    }

    pub fn write_docs_for_path(&self, out: &mut String, path: &mut Path<'static, DocsWrittenFlag>) {
        for i in 0..path.items().len() {
            let docs_written = path.metadata()[i];

            if !docs_written {
                if let Some(docs) = self.docs.get(&path.items()[0..=i]) {
                    write_path_item_docs(out, docs, i);
                }
            }

            path.metadata_mut()[i] = true;
        }
    }
}

fn write_path_item_docs(out: &mut String, docs: &str, nesting_level: usize) {
    let (header, body) = split_header_and_body(docs);

    write_header(out, header, nesting_level);

    for line in body.lines() {
        if let Some(gt_sign_pos) = doc_line_leading_gt_sign_pos(line) {
            let mut line = line.to_string();

            MarkdowDocLineEscape.escape(&mut line, gt_sign_pos);

            out.push_str(&line);
        } else {
            out.push_str(line);
        }

        out.push('\n');
    }

    if !body.is_empty() {
        out.push('\n');
    }
}

fn split_header_and_body(docs: &str) -> (&str, &str) {
    docs.split_once("\n\n").unwrap_or_else(|| (docs, ""))
}

fn write_header(out: &mut String, header: &str, nesting_level: usize) {
    const HEADER_DECORATORS: [(&str, &str, &str); 6] = [
        ("# ", "<h1>\n", "\n</h1>"),
        ("## ", "<h2>\n", "\n</h2>"),
        ("### ", "<h3>\n", "\n</h3>"),
        ("#### ", "<h4>\n", "\n</h4>"),
        ("##### ", "<h5>\n", "\n</h5>"),
        ("###### ", "<h6>\n", "\n</h6>"),
    ];

    let header_decorator_idx = nesting_level.min(HEADER_DECORATORS.len() - 1);
    let header_decorator = HEADER_DECORATORS[header_decorator_idx];
    let is_multiline = header.contains('\n');

    if is_multiline {
        out.push_str(header_decorator.1);
        header.lines().for_each(|line| out.push_str(line.trim()));
        out.push_str(header_decorator.2);
    } else {
        out.push_str(header_decorator.0);
        out.push_str(header.trim());
    }

    out.push_str("\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_docs_for_path() {
        let mut path: Path<'static, ()> = Default::default();
        let mut docs = HashMap::default();

        let mut add_docs = |field: &'static str, doc: String| {
            path.push_struct_field_name(field);
            docs.insert(path.items().to_vec(), doc);
        };

        add_docs(
            "foo",
            [
                "   This is docs for `foo` field",
                "",
                "This is some description of the the `foo` field.",
                "  > this line should be escaped",
            ]
            .join("\n"),
        );

        add_docs("bar", ["This is docs for `bar` field", ""].join("\n"));

        add_docs(
            "baz",
            [
                "This is docs for `baz` field",
                "",
                "Some long",
                "multiline description",
            ]
            .join("\n"),
        );

        add_docs(
            "qux",
            [
                "This is docs for `qux` field",
                "",
                "It has also has",
                "long multiline description",
            ]
            .join("\n"),
        );

        let writer = DocWriter::new(docs);
        let mut path: Path<'static, DocsWrittenFlag> = Default::default();

        path.push_struct_field_name("foo");
        path.push_struct_field_name("bar");
        path.push_struct_field_name("baz");

        let mut out = String::new();

        writer.write_docs_for_path(&mut out, &mut path);

        assert_eq!(
            out,
            [
                "# This is docs for `foo` field",
                "",
                "This is some description of the the `foo` field.",
                "<span>&gt;</span> this line should be escaped",
                "",
                "## This is docs for `bar` field",
                "",
                "### This is docs for `baz` field",
                "",
                "Some long",
                "multiline description",
                "",
                ""
            ]
            .join("\n")
        );

        path.push_struct_field_name("qux");

        let mut out = String::new();

        writer.write_docs_for_path(&mut out, &mut path);

        assert_eq!(
            out,
            [
                "#### This is docs for `qux` field",
                "",
                "It has also has",
                "long multiline description",
                "",
                ""
            ]
            .join("\n"),
        );
    }
}
