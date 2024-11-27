use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum CacheError {
    #[error("{0}")]
    ConnectionError(String),
    #[error("{0}")]
    NotFound(String),
}