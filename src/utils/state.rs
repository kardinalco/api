use super::config::Config;
use crate::exceptions::cache::CacheError;
use crate::exceptions::db::DatabaseError;
use crate::exceptions::error::Error;
use crate::services::permission::Permission;
use actix_session::storage::RedisSessionStore;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::{Database, DatabaseConnection};
use std::str::FromStr;
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Config,
    pub session_store: RedisSessionStore,
    pub cache: Pool<RedisConnectionManager>,
    pub permission: Permission,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        let config = Config::new()?;
        Self::initialize_logger(&config);
        let cache = build_cache(&config.redis.url).await?;
        let db = build_db(&config.database.url).await?;
        Ok(Self {
            permission: build_permission(db.clone(), cache.clone()),
            session_store: build_redis_session_store(&config.redis.url).await?,
            cache,
            db,
            settings: config,
        })
    }

    pub fn initialize_logger(settings: &Config) {
        /*let provider = TracerProvider::builder()
            .with_simple_exporter(opentelemetry_stdout::SpanExporter::default())
            .build();
        let tracer = provider.tracer("kardinal");*/
        tracing_subscriber::registry()
            .with(fmt::layer())
            //.with(tracing_opentelemetry::layer().with_tracer(tracer))
            .with(
                EnvFilter::from_str(&settings.clone().log.level.as_str())
                    .unwrap_or(Default::default()),
            )
            .init();
    }
}

pub fn build_permission(db: DatabaseConnection, cache: Pool<RedisConnectionManager>) -> Permission {
    Permission::new(Arc::new(db), Arc::new(cache))
}

pub async fn build_cache(url: &str) -> Result<Pool<RedisConnectionManager>, Error> {
    let manager = RedisConnectionManager::new(url)
        .map_err(|e| Error::Cache(CacheError::ConnectionError(e.to_string())))?;
    Ok(Pool::builder()
        .build(manager)
        .await
        .map_err(|e| CacheError::ConnectionError(e.to_string()))?)
}

pub async fn build_db(url: &str) -> Result<DatabaseConnection, DatabaseError> {
    Ok(Database::connect(url).await?)
}

pub async fn build_redis_session_store(url: &String) -> Result<RedisSessionStore, Error> {
    Ok(RedisSessionStore::new(format!("{}/1", url)).await?)
}
