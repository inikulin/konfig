pub mod error;
pub mod parser;
pub mod ser;
pub mod value;

use crate::error::Result;
use crate::ser::Serializer;
use serde::Serialize;

pub use self::value::Value;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize + ?Sized,
{
    let mut out = String::with_capacity(128);
    let mut ser = Serializer::new(&mut out);

    value.serialize(&mut ser)?;

    Ok(out)
}
