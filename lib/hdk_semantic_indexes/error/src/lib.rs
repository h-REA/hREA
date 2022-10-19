use thiserror::*;
use std::string::FromUtf8Error;
use holo_hash::EntryHash;

/// Custom error types for handling semantic indexing errors.
///
#[derive(Error, Debug, Clone)]
pub enum SemanticIndexError {
    #[error("No results found")]
    EmptyQuery,

    #[error("No index found at address {0}")]
    IndexNotFound(EntryHash),
    #[error("Index at address {0} failed parsing with error {1}")]
    CorruptIndexError(EntryHash, String),
    #[error("String index with malformed bytes {0:?}")]
    BadStringIndexError(Vec<u8>),
    #[error("Time indexing error {0}")]
    BadTimeIndexError(String),
}

impl From<FromUtf8Error> for SemanticIndexError {
    fn from(e: FromUtf8Error) -> SemanticIndexError {
        SemanticIndexError::BadStringIndexError(e.into_bytes())
    }
}
