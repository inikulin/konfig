#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

#[doc(inline)]
pub use konfig_edit::error;

#[doc(inline)]
pub use konfig_edit::parser;

#[doc(inline)]
pub use konfig_edit::value;

#[doc(inline)]
#[cfg(feature = "serde")]
pub use konfig_serde::ser;

#[doc(inline)]
#[cfg(feature = "serde")]
pub use konfig_serde::de;

#[doc(inline)]
pub use self::value::{Value, ValueCell};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use konfig_edit::value::{from_value, to_value};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use konfig_serde::ser::{to_string, Serializer};

#[doc(inline)]
#[cfg(feature = "serde")]
pub use konfig_serde::de::{from_str, Deserializer};

#[doc(inline)]
pub use konfig_edit::error::{Error, ParseError, Result};

#[doc(inline)]
pub use konfig_edit::parser::parse;

#[cfg(feature = "macros")]
pub use konfig_macros::konfig;
