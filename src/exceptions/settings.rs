use config::ConfigError;
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum SettingsError {
    #[error("Missing configuration {0}")]
    MissingConfiguration(String)
}

impl From<ConfigError> for SettingsError {
    fn from(value: ConfigError) -> Self {
        match value {
            ConfigError::NotFound(a) => Self::MissingConfiguration(a),
            ConfigError::Message(a) => Self::MissingConfiguration(a),
            _ => Self::MissingConfiguration(String::from("TODO"))
        }
    }
}