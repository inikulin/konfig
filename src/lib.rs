#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

pub mod error;
pub mod parser;
pub mod value;

#[cfg(feature = "serde")]
pub mod ser;

#[cfg(feature = "serde")]
pub mod de;

#[doc(inline)]
pub use self::value::{Value, ValueCell};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use self::value::{from_value, to_value};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use self::ser::{to_string, Serializer};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use self::de::{from_str, Deserializer};

#[doc(inline)]
pub use self::error::{Error, ParseError, Result};

#[doc(inline)]
pub use self::parser::parse;
