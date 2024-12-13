use super::config::Config;
use crate::exceptions::cache::CacheError;
use crate::exceptions::db::DatabaseError;
use crate::exceptions::error::Error;
use crate::services::permission::Permission;
use actix_session::storage::RedisSessionStore;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Sampler;
use tracing::log;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Layer};

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
        let cache = build_cache(&config.redis_url).await?;
        let db = build_db(&config.database_url).await?;
        Ok(Self {
            permission: build_permission(db.clone(), cache.clone()),
            session_store: build_redis_session_store(&config.redis_url).await?,
            cache,
            db,
            settings: config,
        })
    }

    pub fn initialize_logger(settings: &Config) {
        let exporter = opentelemetry_otlp::SpanExporter::builder()
            .with_tonic()
            .with_endpoint(&settings.opentelemetry_url)
            .with_timeout(Duration::from_secs(3))
            .build().unwrap();
        let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
            .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
            .with_resource(opentelemetry_sdk::Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", "API"),
            ]))
            .with_sampler(Sampler::AlwaysOn)
            .build();
        let tracer = tracer_provider.tracer("kardinal");
        let fmt_layer = fmt::layer()
            .with_filter(EnvFilter::from_str(&settings.clone().log.as_str()).unwrap());
        tracing_subscriber::registry()
            .with(OpenTelemetryLayer::new(tracer)
                .with_filter(EnvFilter::from_str(&settings.clone().log.as_str()).unwrap()))
            .with(fmt_layer)
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
    let mut options = ConnectOptions::new(url);
    options.sqlx_logging_level(log::LevelFilter::Info);
    options.max_connections(16);
    options.connect_lazy(false);
    Ok(Database::connect(options).await?)
}

pub async fn build_redis_session_store(url: &String) -> Result<RedisSessionStore, Error> {
    Ok(RedisSessionStore::new(format!("{}/1", url)).await?)
}
