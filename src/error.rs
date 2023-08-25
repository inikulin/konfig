use crate::parser::ParseError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("map key should be a string or index")]
    InvalidMapKeyType,
    #[error("tuples are unsupported")]
    TuplesUnsupported,
    #[error("structure enum variants are unsupported")]
    StructVariantsUnsupported,
    #[error("{0}")]
    Parsing(ParseError),
    #[error("{0}")]
    Custom(String),
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
