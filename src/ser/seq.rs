use super::introspector::{Introspector, ValueKind};
use super::kv::{KVSerializer, KVSerializerMode};
use super::utils;
use super::Serializer;
use crate::error::{Error, Result};
use serde::ser::Serialize;

enum SeqRepresentation<'s, 'o> {
    // NOTE: wrapped in `Option`, so we can convert to KV without resorting to unsafe code.
    Inline(Option<&'s mut Serializer<'o>>),
    Kv(KVSerializer<'s, 'o>),
}

impl<'s, 'o> SeqRepresentation<'s, 'o> {
    fn switch_to_kv(&mut self) {
        let serializer = match self {
            Self::Inline(s) => s.take().expect("should have inline serializer"),
            _ => unreachable!("serializer can be switched to kv only once"),
        };

        *self = SeqRepresentation::Kv(KVSerializer::new(serializer, KVSerializerMode::Default));
    }
}

pub struct SeqSerializer<'s, 'o> {
    repr: SeqRepresentation<'s, 'o>,
    current_index: usize,
}

impl<'s, 'o> SeqSerializer<'s, 'o> {
    pub(super) fn new(serializer: &'s mut Serializer<'o>) -> Self {
        Self {
            repr: SeqRepresentation::Inline(Some(serializer)),
            current_index: 0,
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
        if self.current_index == 0 && Introspector::val_kind(value) != ValueKind::Leaf {
            self.repr.switch_to_kv();
        }

        match self.repr {
            SeqRepresentation::Kv(ref mut serializer) => {
                let key = utils::make_map_key(|key| {
                    utils::write_int(key, self.current_index);
                    Ok(())
                })?;

                serializer.inner.push_path(key);

                serde::ser::SerializeMap::serialize_value(serializer, value)?;
            }
            SeqRepresentation::Inline(Some(ref mut serializer)) => {
                if self.current_index == 0 {
                    serializer.serialize_breadcrumbs();
                    serializer.out.push('[');
                } else {
                    serializer.out.push_str(", ");
                }

                serializer.skip_breadcrumbs_serialization = true;
                value.serialize(&mut **serializer)?;
                serializer.skip_breadcrumbs_serialization = false;
            }
            SeqRepresentation::Inline(None) => unreachable!("inline repr should have serializer"),
        }

        self.current_index += 1;

        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.current_index == 0 {
            let SeqRepresentation::Inline(Some(serializer)) = self.repr else {
                unreachable!("repr shouldn't be switched to kv for empty sequences")
            };

            serializer.serialize_breadcrumbs();
            serializer.out.push_str("[]");
        } else {
            match self.repr {
                SeqRepresentation::Kv(serializer) => serde::ser::SerializeStruct::end(serializer)?,
                SeqRepresentation::Inline(Some(serializer)) => serializer.out.push(']'),
                SeqRepresentation::Inline(None) => {
                    unreachable!("inline repr should have serializer")
                }
            }
        }

        Ok(())
    }
}
