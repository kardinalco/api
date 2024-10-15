use deadpool_redis::{ConfigError, CreatePoolError, PoolError};
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum CacheError {
    #[error("")]
    ConnectionError,
}

impl From<diesel::result::Error> for CacheError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            _ => CacheError::ConnectionError,
        }
    }
}

impl From<r2d2_redis::redis::RedisError> for CacheError {
    fn from(value: r2d2_redis::redis::RedisError) -> Self {
        todo!()
    }
}

impl From<r2d2::Error> for CacheError {
    fn from(value: r2d2::Error) -> Self {
        todo!()
    }
}

impl From<CreatePoolError> for CacheError {
    fn from(value: CreatePoolError) -> Self {
        todo!()
    }
}