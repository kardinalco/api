use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use config::ConfigError;
use s3::creds::error::CredentialsError;
use s3::error::S3Error;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum SettingsError {
    #[error("Missing configuration {0}")]
    MissingConfiguration(String),
    #[error("Bucket name {0} not found !")]
    WrongBucketName(String),
    #[error("Wrong bucket credentials {0}")]
    WrongBucketCredentials(String)
}

impl ResponseError for SettingsError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        println!("{:?}", self);
        HttpResponse::InternalServerError().json(json!({"message": "An internal error occurred !"}))
    }
}

impl From<ConfigError> for SettingsError {
    fn from(value: ConfigError) -> Self {
        match value {
            ConfigError::NotFound(a) => Self::MissingConfiguration(a),
            ConfigError::Message(a) => Self::MissingConfiguration(a),
            _ => Self::MissingConfiguration(String::from("TODO")),
        }
    }
}


impl From<CredentialsError> for SettingsError {
    fn from(value: CredentialsError) -> Self {
        match value {
            CredentialsError::ConfigNotFound => Self::WrongBucketCredentials(String::from("Config not found !")),
            CredentialsError::ConfigMissingAccessKeyId => Self::WrongBucketCredentials(String::from("Access Key not found !")),
            CredentialsError::ConfigMissingSecretKey => Self::WrongBucketCredentials(String::from("Secret Key not found !")),
            _ => {
                println!("{:?}", value);
                Self::WrongBucketCredentials(String::from("TODO"))
            },
        }
    }
}

impl From<S3Error> for SettingsError {
    fn from(value: S3Error) -> Self {
        match value {
            _ => {
                println!("{:?}", value);
                Self::WrongBucketCredentials(String::from("TODO"))
            },
        }
    }
}