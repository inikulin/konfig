pub(crate) mod doc_writer;
mod with_docs;

use crate::ser::Serializer;
use konfig_edit::error::Result;
use serde::ser::Serialize;

pub use self::with_docs::WithDocs;

pub fn to_string_with_docs<T>(value: &T) -> Result<String>
where
    T: Serialize + WithDocs + ?Sized,
{
    let mut out = String::with_capacity(128);
    let mut path = Default::default();
    let mut docs = Default::default();

    value.add_docs(&mut path, &mut docs)?;

    let mut ser = Serializer::new_with_docs(&mut out, docs);

    value.serialize(&mut ser)?;

    Ok(out)
}
