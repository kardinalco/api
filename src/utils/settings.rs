use crate::services::cache::CachedSettings;
use redis::{ErrorKind, FromRedisValue};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use settings::global::Global;
use settings::google::Google;
use settings::cache::Cache;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings<T>(T);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecretSettings<T>(T);

impl<T: DeserializeOwned> FromRedisValue for Settings<T> {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let s: String = redis::FromRedisValue::from_redis_value(v)?;
        serde_json::from_str(&s)
            .map_err(|_| redis::RedisError::from((ErrorKind::TypeError, "Cannot parse value")))
    }
}

impl<T> Settings<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl CachedSettings for Settings<Google> {
    fn get_key<'a>() -> &'a str {
        "google"
    }
}

impl CachedSettings for Settings<Global> {
    fn get_key<'a>() -> &'a str {
        "global"
    }
}

impl CachedSettings for Settings<Cache> {
    fn get_key<'a>() -> &'a str {
        "cache"
    }
}