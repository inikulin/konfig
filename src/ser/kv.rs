use super::introspector::{Introspector, ValueKind};
use super::map_key::MapKeySerializer;
use super::Serializer;
use crate::error::{Error, Result};
use serde::ser::Serialize;

#[derive(PartialEq)]
pub(super) enum KVSerializerMode {
    Default,
    WithPathPopOnCompletion,
    Noop,
}

pub struct KVSerializer<'s, 'o> {
    pub(super) inner: &'s mut Serializer<'o>,
    mode: KVSerializerMode,
}

impl<'s, 'o> KVSerializer<'s, 'o> {
    pub(super) fn new(inner: &'s mut Serializer<'o>, mode: KVSerializerMode) -> Self {
        Self { inner, mode }
    }
}

impl<'s, 'o> serde::ser::SerializeMap for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.mode != KVSerializerMode::Noop {
            self.inner.push_path(MapKeySerializer::serialize(key)?);
        }

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        match Introspector::val_kind(value) {
            ValueKind::Leaf | ValueKind::KvOnlyLeaf => value.serialize(&mut *self.inner)?,
            _ => value.serialize(&mut *self.inner)?,
        }

        self.inner.pop_path();

        Ok(())
    }

    #[inline]
    fn end(self) -> Result<()> {
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        if self.mode == KVSerializerMode::WithPathPopOnCompletion {
            self.inner.pop_path();
        }

        Ok(())
    }
}

impl<'s, 'o> serde::ser::SerializeStruct for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.mode == KVSerializerMode::Noop {
            return Ok(());
        }

        self.inner.push_path(key);

        serde::ser::SerializeMap::serialize_value(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        serde::ser::SerializeMap::end(self)
    }
}

impl<'s, 'o> serde::ser::SerializeStructVariant for KVSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        serde::ser::SerializeStruct::serialize_field(self, key, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        serde::ser::SerializeMap::end(self)
    }
}
