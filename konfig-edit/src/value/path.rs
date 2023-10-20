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
pub struct Path<'i, M>(Vec<(PathItem<'i>, M)>);

impl<'i, M> Path<'i, M> {
    #[inline]
    pub fn items(&self) -> &[(PathItem, M)] {
        &self.0
    }

    #[inline]
    pub fn push(&mut self, item: PathItem<'i>, meta: M) {
        self.0.push((item, meta));
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(PathItem, M)> {
        self.0.pop()
    }

    #[inline]
    pub fn push_sequence_index(&mut self, idx: usize, meta: M) {
        self.push(PathItem::SequenceIndex(idx), meta);
    }

    #[inline]
    pub fn push_map_key(&mut self, key: impl Into<Cow<'i, str>>, meta: M) {
        self.push(PathItem::MapKey(key.into()), meta);
    }

    #[inline]
    pub fn push_struct_field_name(&mut self, name: impl Into<Cow<'i, str>>, meta: M) {
        self.push(PathItem::StructFieldName(name.into()), meta);
    }

    #[inline]
    pub fn push_variant_name(&mut self, name: impl Into<Cow<'i, str>>, meta: M) {
        self.push(PathItem::VariantName(name.into()), meta)
    }

    pub fn write(&self, out: &mut impl Write) -> fmt::Result {
        out.write_char('>')?;

        for (idx, item) in self.0.iter().enumerate() {
            out.write_char(' ')?;

            if idx != 0 {
                out.write_str("> ")?;
            }

            item.0.write(out)?;
        }

        Ok(())
    }
}

impl<M> fmt::Display for Path<'_, M> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}
