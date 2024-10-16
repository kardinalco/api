use config::{Config, Environment};
use serde::{Serialize, Deserialize};

use crate::exceptions::{error::Error, settings::SettingsError};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Database {
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

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub database: Database,
    pub log: Log,
}

impl Settings {
    pub fn new() -> Result<Self, SettingsError> {
        Ok(Config::builder().add_source(Environment::with_prefix("").separator("_")).build()?.try_deserialize()?)
    }
}