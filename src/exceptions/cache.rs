use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum CacheError {
    #[error("")]
    ConnectionError,
}