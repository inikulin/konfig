#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

pub mod error;
pub mod parser;
pub mod value;

#[doc(hidden)]
pub mod serialization_utils;
