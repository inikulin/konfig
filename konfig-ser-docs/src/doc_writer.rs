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
                    let is_last_path_item = i == path.items().len() - 1;

                    write_path_item_docs(out, docs, i, is_last_path_item);
                }
            }

            path.metadata_mut()[i] = true;
        }
    }
}

fn write_path_item_docs(
    out: &mut String,
    docs: &str,
    nesting_level: usize,
    is_last_path_item: bool,
) {
    let mut is_doc_head = true;

    for (idx, line) in docs.lines().enumerate() {
        if line.trim().is_empty() {
            is_doc_head = false;
        }

        if is_last_path_item {
            if idx == 0 {
                out.push_str("---\n");
            }
        } else if is_doc_head {
            write_header_line(out, line, nesting_level);
            continue;
        }

        if let Some(gt_sign_pos) = doc_line_leading_gt_sign_pos(line) {
            let mut line = line.to_string();

            MarkdowDocLineEscape.escape(&mut line, gt_sign_pos);

            out.push_str(&line);
        } else {
            out.push_str(line);
        }

        out.push('\n');
    }

    out.push('\n');
}

fn write_header_line(out: &mut String, line: &str, nesting_level: usize) {
    const HEADER_PREFIXES: [&str; 6] = ["# ", "## ", "### ", "#### ", "##### ", "###### "];

    let header_prefix_idx = nesting_level.min(HEADER_PREFIXES.len() - 1);

    out.push_str(HEADER_PREFIXES[header_prefix_idx]);
    out.push_str(line);
    out.push('\n');
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
                "This is docs for `foo` field",
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
                "  <span>&gt;</span> this line should be escaped",
                "",
                "## This is docs for `bar` field",
                "",
                "---",
                "This is docs for `baz` field",
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
                "---",
                "This is docs for `qux` field",
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
