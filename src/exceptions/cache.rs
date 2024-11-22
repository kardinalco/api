use serde::Serialize;
use crate::exceptions::error::Error;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum CacheError {
    #[error("{0}")]
    ConnectionError(String),
}