use actix_session::storage::RedisSessionStore;
use sea_orm::{Database, DatabaseConnection};

use crate::exceptions::error::Error;
use crate::exceptions::db::DatabaseError;

use super::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Settings,
    pub session_store: RedisSessionStore,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        let settings = Settings::new()?;
        Ok(Self {
            db: build_db(&settings.database.url).await?,
            session_store: build_redis_session_store(&settings.redis.url).await?,
            settings,
        })
    }
}

pub async fn build_db(url: &String) -> Result<DatabaseConnection, DatabaseError> {
    Ok(Database::connect(url).await?)
}

pub async fn build_redis_session_store(url: &String) -> Result<RedisSessionStore, Error> {
    Ok(RedisSessionStore::new("/***/").await?)
}