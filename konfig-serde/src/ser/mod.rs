mod is_primitive;
mod kv;
mod map_key;
mod seq;
mod serializer;

pub mod doc_format;

use konfig_edit::error::Result;
use serde::ser::Serialize;

pub use self::serializer::Serializer;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let mut out = String::with_capacity(128);
    let mut ser = Serializer::new(&mut out, None);

    value.serialize(&mut ser)?;

    Ok(out)
}
