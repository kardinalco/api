use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum CacheError {
    #[error("{0}")]
    ConnectionError(String),
}

