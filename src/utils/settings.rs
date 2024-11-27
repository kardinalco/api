use config::{Config, Environment};
use serde::{Serialize, Deserialize};

use crate::exceptions::settings::SettingsError;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Database {
    pub url: String
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Redis {
    pub url: String
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Log {
    pub level: LogLevel
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub enum LogLevel {
    #[default]
    Debug,
    Info,
    Warning,
    Error
}

impl LogLevel {
    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warning => "warning",
            LogLevel::Error => "error"
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Keys {
    pub session: String,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Api {
    pub port: i32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub database: Database,
    pub log: Log,
    pub redis: Redis,
    pub keys: Keys,
    pub api: Api
}

impl Settings {
    pub fn new() -> Result<Self, SettingsError> {
        Ok(Config::builder().add_source(Environment::default().separator("__")).build()?.try_deserialize()?)
    }
}