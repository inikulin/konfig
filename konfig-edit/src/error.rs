pub use crate::parser::error::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("map key should be a string or index")]
    InvalidMapKeyType,
    #[error("128-bit integers are not supported")]
    Int128NotSupported,
    #[error("infinite and NaN floating point numbers are not supported")]
    InfAndNanNotSupported,
    #[error("{0}")]
    Parsing(ParseError),
    #[error("merge conflict at path: {path}")]
    MergeConflict { path: String },
    #[error("invalid field name or enum variant: {0}")]
    InvalidFieldNameOrEnumVariant(String),
    #[error("{0}")]
    Custom(String),
}

impl Error {
    #[inline]
    pub fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl Error {
    #[inline]
    pub(crate) fn de_fewer_elements_in_seq(actual_len: usize) -> Self {
        serde::de::Error::invalid_length(actual_len, &"fewer elements in sequence")
    }

    #[inline]
    pub(crate) fn de_fewer_elements_in_map(len: usize) -> Self {
        serde::de::Error::invalid_length(len, &"fewer elements in map")
    }

    #[inline]
    pub(crate) fn de_map_value_missing() -> Self {
        serde::de::Error::custom("value is missing")
    }

    #[inline]
    pub(crate) fn de_expected_newtype_variant(got: serde::de::Unexpected) -> Self {
        serde::de::Error::invalid_type(got, &"newtype variant")
    }

    #[inline]
    pub(crate) fn de_expected_unit_variant(got: serde::de::Unexpected) -> Self {
        serde::de::Error::invalid_type(got, &"unit variant")
    }

    #[inline]
    pub(crate) fn de_expected_tuple_variant(got: serde::de::Unexpected) -> Self {
        serde::de::Error::invalid_type(got, &"tuple variant")
    }

    #[inline]
    pub(crate) fn de_expected_struct_variant(got: serde::de::Unexpected) -> Self {
        serde::de::Error::invalid_type(got, &"struct variant")
    }
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg)
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::custom(msg)
    }
}
