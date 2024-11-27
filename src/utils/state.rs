use std::str::FromStr;
use actix_session::storage::RedisSessionStore;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::exceptions::error::Error;
use crate::exceptions::db::DatabaseError;
use crate::exceptions::cache::CacheError;
use super::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Settings,
    pub session_store: RedisSessionStore,
    pub cache: Pool<RedisConnectionManager>
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        let settings = Settings::new()?;
        Self::initialize_logger(&settings);
        Ok(Self {
            db: build_db(&settings.database.url).await?,
            cache: build_cache(&settings.redis.url).await?,
            session_store: build_redis_session_store(&settings.redis.url).await?,
            settings,
        })
    }

    pub fn initialize_logger(settings: &Settings) {
        /*let provider = TracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build();
        let tracer = provider.tracer("kardinal");*/
        tracing_subscriber::registry()
            .with(fmt::layer())
            //.with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(EnvFilter::from_str(&settings.clone().log.level.as_str()).unwrap_or(Default::default()))
            .init();
    }

}

pub async fn build_cache(url: &str) -> Result<Pool<RedisConnectionManager>, Error> {
    let manager = RedisConnectionManager::new(url)
        .map_err(|e| Error::Cache(CacheError::ConnectionError(e.to_string())))?;
    Ok(Pool::builder().build(manager).await.map_err(|e| CacheError::ConnectionError(e.to_string()))?)
}

pub async fn build_db(url: &str) -> Result<DatabaseConnection, DatabaseError> {
    Ok(Database::connect(url).await?)
}

pub async fn build_redis_session_store(url: &String) -> Result<RedisSessionStore, Error> {
    Ok(RedisSessionStore::new(url).await?)
}