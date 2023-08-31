use super::is_primitive::is_primitive;
use super::kv::KVSerializer;
use super::utils;
use super::Serializer;
use crate::error::{Error, Result};
use serde::ser::Serialize;

enum SeqRepresentation<'s, 'o> {
    Inline {
        // NOTE: wrapped in `Option`, so we can convert to KV without resorting to unsafe code.
        inner: Option<&'s mut Serializer<'o>>,
        serialized_primitives: Vec<String>,
    },
    Kv(KVSerializer<'s, 'o>),
}

impl<'s, 'o> SeqRepresentation<'s, 'o> {
    fn switch_to_kv(&mut self) -> Result<()> {
        let Self::Inline {
            inner,
            serialized_primitives,
        } = self
        else {
            unreachable!("serializer can be switched to kv only once")
        };

        let serializer = inner.take().expect("should have inline serializer");

        serialized_primitives
            .iter()
            .enumerate()
            .try_for_each(|(idx, value)| {
                push_path_for_idx(serializer, idx)?;
                serializer.serialize_path();
                serializer.out.push_str(value);
                serializer.pop_path();

                Ok(())
            })?;

        *self = SeqRepresentation::Kv(KVSerializer::new(serializer));

        Ok(())
    }
}

pub struct SeqSerializer<'s, 'o> {
    repr: SeqRepresentation<'s, 'o>,
    current_index: usize,
}

impl<'s, 'o> SeqSerializer<'s, 'o> {
    pub(super) fn new(inner: &'s mut Serializer<'o>) -> Self {
        Self {
            repr: SeqRepresentation::Inline {
                inner: Some(inner),
                serialized_primitives: vec![],
            },
            current_index: 0,
        }
    }

    fn ensure_inline_repr_serialized(&mut self) {
        if let SeqRepresentation::Inline {
            inner: Some(ref mut inner),
            ref mut serialized_primitives,
        } = self.repr
        {
            inner.serialize_path();
            inner.out.push('[');
            inner.out.push_str(&serialized_primitives.join(", "));
            inner.out.push(']');
        }
    }
}

impl<'s, 'o> serde::ser::SerializeSeq for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let SeqRepresentation::Inline {
            ref mut serialized_primitives,
            ..
        } = self.repr
        {
            if is_primitive(value) {
                serialized_primitives.push(serialize_primitive(value)?);
                self.current_index += 1;

                return Ok(());
            }

            self.repr.switch_to_kv()?;
        }

        if let SeqRepresentation::Kv(ref mut inner) = self.repr {
            push_path_for_idx(inner.inner, self.current_index)?;
            serde::ser::SerializeMap::serialize_value(inner, value)?;
        }

        self.current_index += 1;

        Ok(())
    }

    fn end(mut self) -> Result<()> {
        self.ensure_inline_repr_serialized();

        Ok(())
    }
}

impl<'s, 'o> serde::ser::SerializeTuple for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'s, 'o> serde::ser::SerializeTupleStruct for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    #[inline]
    fn end(self) -> Result<()> {
        serde::ser::SerializeSeq::end(self)
    }
}

impl<'s, 'o> serde::ser::SerializeTupleVariant for SeqSerializer<'s, 'o> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        serde::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(mut self) -> Result<()> {
        self.ensure_inline_repr_serialized();

        let inner = match self.repr {
            SeqRepresentation::Inline {
                inner: Some(inner), ..
            } => inner,
            SeqRepresentation::Kv(KVSerializer { inner }) => inner,
            _ => unreachable!(),
        };

        inner.pop_path();

        Ok(())
    }
}

fn serialize_primitive<T>(value: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let mut out = String::with_capacity(16);
    let mut serializer = Serializer::new(&mut out);

    serializer.skip_path_serialization = true;
    value.serialize(&mut serializer)?;

    Ok(out)
}

fn push_path_for_idx(serializer: &mut Serializer, idx: usize) -> Result<()> {
    let key = utils::make_map_key(|key| {
        utils::write_int(key, idx);
        Ok(())
    })?;

    serializer.push_path(key);

    Ok(())
}
