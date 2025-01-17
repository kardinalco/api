use super::auth::AuthenticateError;
use super::cache::CacheError;
use super::db::DatabaseError;
use super::settings::SettingsError;
use crate::exceptions::auth::AuthenticateError::CannotCreateUserSession;
use crate::exceptions::entity::EntityError;
use crate::exceptions::request::RequestError;
use actix_session::{SessionGetError, SessionInsertError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, ResponseError};
use bb8::RunError;
use redis::RedisError;
use s3::creds::error::CredentialsError;
use s3::error::S3Error;
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;
use tracing::error;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Auth(#[from] AuthenticateError),

    #[error("Internal Server Error: {0}")]
    InternalServer(String),

    #[error("Parse request body exception !")]
    Parse(String),

    #[error("{0}")]
    Database(#[from] DatabaseError),

    #[error("{0}")]
    Cache(#[from] CacheError),

    #[error("{0}")]
    Settings(#[from] SettingsError),

    #[error("{0}")]
    Entity(#[from] EntityError),

    #[error("Hash error !")]
    Hash,

    #[error("{0}")]
    Request(RequestError),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Auth(e) => e.status_code(),
            Error::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Parse(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Error::Auth(e) => e.error_response(),
            Error::InternalServer(_) => HttpResponse::InternalServerError().finish().into(),
            Error::Parse(_) => HttpResponse::BadRequest().json(json!(self)),
            Error::Database(e) => e.error_response(),
            Error::Entity(e) => e.error_response(),
            Error::Request(e) => e.error_response(),
            Error::Settings(e) => e.error_response(),
            _ => unimplemented!(),
        }
    }
}

impl Responder for Error {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse {
        match self {
            Error::Auth(e) => e.error_response(),
            Error::InternalServer(x) => Error::InternalServer(x).error_response(),
            Error::Parse(x) => Error::Parse(x).error_response(),
            _ => unimplemented!(),
        }
    }
}

impl From<bcrypt::BcryptError> for Error {
    fn from(value: bcrypt::BcryptError) -> Self {
        error!("BcryptError: {:?}", value);
        Error::Hash
    }
}

impl From<DbErr> for Error {
    fn from(e: DbErr) -> Self {
        error!("DbErr: {:?}", e);
        Error::Database(DatabaseError::from(e))
    }
}

impl From<actix_web::Error> for Error {
    fn from(e: actix_web::Error) -> Self {
        Error::InternalServer(e.to_string())
    }
}

impl From<SessionInsertError> for Error {
    fn from(value: SessionInsertError) -> Self {
        error!("SessionInsertError: {:?}", value);
        Error::Auth(CannotCreateUserSession)
    }
}

impl From<SessionGetError> for Error {
    fn from(value: SessionGetError) -> Self {
        error!("SessionGetError: {:?}", value);
        Error::Auth(CannotCreateUserSession)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        error!("IoError: {:?}", e);
        Error::InternalServer(e.to_string())
    }
}

impl From<anyhow::Error> for CacheError {
    fn from(v: anyhow::Error) -> Self {
        error!("AnyhowError: {:?}", v);
        CacheError::ConnectionError(v.to_string())
    }
}

impl From<RunError<RedisError>> for Error {
    fn from(value: RunError<RedisError>) -> Self {
        error!("RunError: {:?}", value);
        Error::InternalServer(value.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        error!("AnyhowError: {:?}", value);
        Error::InternalServer(value.to_string())
    }
}

impl From<RedisError> for Error {
    fn from(value: RedisError) -> Self {
        error!("RedisError: {:?}", value);
        match value.kind() {
            redis::ErrorKind::TypeError => Error::InternalServer("Cannot parse value".to_string()),
            _ => Error::InternalServer("Unknown error".to_string()),
        }
    }
}

impl From<RequestError> for Error {
    fn from(value: RequestError) -> Self {
        Error::Request(value)
    }
}

impl From<S3Error> for Error {
    fn from(value: S3Error) -> Self {
        Error::Settings(SettingsError::from(value))
    }
}

impl From<CredentialsError> for Error {
    fn from(value: CredentialsError) -> Self {
        Error::Settings(SettingsError::from(value))
    }
}