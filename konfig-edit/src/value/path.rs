use crate::serializer::components::{write_escaped_str, write_int};
use std::borrow::Cow;
use std::fmt::{self, Write};

#[derive(Debug, PartialEq)]
pub enum PathItem<'i> {
    SequenceIndex(usize),
    MapKey(Cow<'i, str>),
    StructFieldName(Cow<'i, str>),
    VariantName(Cow<'i, str>),
}

impl PathItem<'_> {
    pub fn write(&self, out: &mut impl Write) -> fmt::Result {
        match self {
            PathItem::MapKey(key) => {
                out.write_str("[\"")?;
                write_escaped_str(out, key)?;
                out.write_str("\"]")
            }
            PathItem::StructFieldName(name) => out.write_str(name),
            PathItem::SequenceIndex(idx) => {
                out.write_char('[')?;
                write_int(out, *idx)?;
                out.write_char(']')
            }
            PathItem::VariantName(name) => {
                out.write_char('`')?;
                out.write_str(name)?;
                out.write_char('`')
            }
        }
    }
}

impl fmt::Display for PathItem<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}

#[derive(Debug, PartialEq, Default)]
pub struct Path<'i>(Vec<PathItem<'i>>);

impl<'i> Path<'i> {
    #[inline]
    pub fn items(&self) -> &[PathItem] {
        &self.0
    }

    #[inline]
    pub fn push(&mut self, item: PathItem<'i>) {
        self.0.push(item);
    }

    #[inline]
    pub fn pop(&mut self) -> Option<PathItem> {
        self.0.pop()
    }

    #[inline]
    pub fn push_sequence_index(&mut self, idx: usize) {
        self.push(PathItem::SequenceIndex(idx));
    }

    #[inline]
    pub fn push_map_key(&mut self, key: impl Into<Cow<'i, str>>) {
        self.push(PathItem::MapKey(key.into()));
    }

    #[inline]
    pub fn push_struct_field_name(&mut self, name: impl Into<Cow<'i, str>>) {
        self.push(PathItem::StructFieldName(name.into()));
    }

    #[inline]
    pub fn push_variant_name(&mut self, name: impl Into<Cow<'i, str>>) {
        self.push(PathItem::VariantName(name.into()))
    }

    pub fn write(&self, out: &mut impl Write) -> fmt::Result {
        out.write_char('>')?;

        for (idx, item) in self.0.iter().enumerate() {
            out.write_char(' ')?;

            if idx != 0 {
                out.write_str("> ")?;
            }

            item.write(out)?;
        }

        Ok(())
    }
}

impl fmt::Display for Path<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}
