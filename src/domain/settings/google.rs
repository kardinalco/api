use redis::{ErrorKind, FromRedisValue};
use serde::{Deserialize, Serialize};
use settings::google::Google;
use crate::services::cache::CachedSettings;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoogleSettings(pub Google);

impl FromRedisValue for GoogleSettings {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let s: String = redis::FromRedisValue::from_redis_value(v)?;
        serde_json::from_str(&s).map_err(|_| { 
            redis::RedisError::from((ErrorKind::TypeError, "Cannot parse value")) 
        })
    }
}

impl CachedSettings for GoogleSettings {
    fn get_key() -> String {
        "google".to_string()
    }
}

impl GoogleSettings {
    pub fn into_inner(self) -> Google {
        self.0
    }
}