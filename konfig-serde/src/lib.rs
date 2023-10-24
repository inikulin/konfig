#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

#[cfg(feature = "ser-docs")]
pub mod ser_docs;

pub mod de;
pub mod ser;
