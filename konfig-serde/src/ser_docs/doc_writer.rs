use konfig_edit::serializer::components::doc_line_leading_gt_sign_pos;
use konfig_edit::serializer::formatting::{DocLineEscape, MarkdowDocLineEscape};
use konfig_edit::value::Path;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct DocWriter {
    docs: HashMap<Path<'static>, String>,
    docs_written_for_path_item: Vec<bool>,
}

impl DocWriter {
    pub fn new_for_path(
        path: &mut Path<'static>,
        docs: HashMap<Path<'static>, String>,
    ) -> Rc<RefCell<Self>> {
        let writer = Rc::new(RefCell::new(Self {
            docs,
            docs_written_for_path_item: vec![],
        }));

        path.set_callbacks(
            {
                let writer = Rc::clone(&writer);

                move || writer.borrow_mut().on_path_item_push()
            },
            {
                let writer = Rc::clone(&writer);

                move || writer.borrow_mut().on_path_item_pop()
            },
        );

        writer
    }

    pub fn write_docs_for_path(&mut self, out: &mut String, path: &mut Path<'static>) {
        for i in 0..path.items().len() {
            if !self.docs_written_for_path_item[i] {
                if let Some(docs) = self.docs.get(&path.items()[0..=i]) {
                    write_path_item_docs(out, docs, i);
                }
            }

            self.docs_written_for_path_item[i] = true;
        }
    }

    pub fn on_path_item_push(&mut self) {
        self.docs_written_for_path_item.push(false);
    }

    pub fn on_path_item_pop(&mut self) {
        self.docs_written_for_path_item.pop();
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
    docs.split_once("\n\n").unwrap_or((docs, ""))
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
    let header = header.trim();
    let is_multiline = header.contains('\n');

    if is_multiline {
        out.push_str(header_decorator.1);
        header.lines().for_each(|line| out.push_str(line.trim()));
        out.push_str(header_decorator.2);
    } else {
        out.push_str(header_decorator.0);
        out.push_str(header);
    }

    out.push_str("\n\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_docs_for_path() {
        let mut path: Path<'static> = Default::default();
        let mut docs = HashMap::default();

        let mut add_docs = |field: &'static str, doc: String| {
            path.push_struct_field_name(field);
            docs.insert(path.clone(), doc);
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

        let mut path: Path<'static> = Default::default();
        let writer = DocWriter::new_for_path(&mut path, docs);

        path.push_struct_field_name("foo");
        path.push_struct_field_name("bar");
        path.push_struct_field_name("baz");

        let mut out = String::new();

        writer.borrow_mut().write_docs_for_path(&mut out, &mut path);

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

        writer.borrow_mut().write_docs_for_path(&mut out, &mut path);

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
