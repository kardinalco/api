use serde::{Deserialize, Serialize};

use crate::exceptions::settings::SettingsError;


#[derive(Serialize, Deserialize, Clone, Default)]
pub enum LogLevel {
    #[default]
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Keys {
    pub session: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Config {
    pub database_url: String,
    pub log: LogLevel,
    pub redis_url: String,
    pub session_key: String,
    pub opentelemetry_url: String,
}

impl Config {
    pub fn new() -> Result<Self, SettingsError> {
        Ok(config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()?)
    }
}