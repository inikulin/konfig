use crate::serializer::components::{write_escaped_str, write_int};
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::{self, Write};
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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

#[derive(Default)]
pub struct Path<'i> {
    items: Vec<PathItem<'i>>,
    on_item_push: Option<Box<dyn Fn()>>,
    on_item_pop: Option<Box<dyn Fn()>>,
}

impl<'i> Path<'i> {
    pub fn set_callbacks(
        &mut self,
        on_item_push: impl Fn() + 'static,
        on_item_pop: impl Fn() + 'static,
    ) {
        self.on_item_push = Some(Box::new(on_item_push));
        self.on_item_pop = Some(Box::new(on_item_pop));
    }

    #[inline]
    pub fn items(&self) -> &[PathItem<'i>] {
        &self.items
    }

    #[inline]
    pub fn push(&mut self, item: PathItem<'i>) {
        self.items.push(item);

        if let Some(on_item_push) = self.on_item_push.as_ref() {
            on_item_push();
        }
    }

    #[inline]
    pub fn pop(&mut self) {
        self.items.pop();

        if let Some(on_item_pop) = self.on_item_pop.as_ref() {
            on_item_pop();
        }
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

        for (idx, item) in self.items.iter().enumerate() {
            out.write_char(' ')?;

            if idx != 0 {
                out.write_str("> ")?;
            }

            item.write(out)?;
        }

        Ok(())
    }
}

impl PartialEq for Path<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
}

impl Eq for Path<'_> {}

impl Hash for Path<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.items.hash(state);
    }
}

impl Clone for Path<'_> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            on_item_push: None,
            on_item_pop: None,
        }
    }
}

impl<'i> Borrow<[PathItem<'i>]> for Path<'i> {
    fn borrow(&self) -> &[PathItem<'i>] {
        &self.items
    }
}

impl fmt::Debug for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Path").field("items", &self.items).finish()
    }
}

impl fmt::Display for Path<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write(f)
    }
}
