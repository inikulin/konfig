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
    #[error("{0}")]
    Custom(String),
}

#[cfg(feature = "serde")]
impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
