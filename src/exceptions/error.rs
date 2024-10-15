use actix_web::{HttpResponse, Responder, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use r2d2_redis::redis::RedisError;
use serde::Serialize;
use serde_json::json;
use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::cache::CacheError;
use crate::exceptions::db::DatabaseError;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    Auth(#[from] AuthenticateError),
    
    #[error("Internal Server Error")]
    InternalServer(String),

    #[error("Parse request body exception !")]
    Parse(String),

    #[error("{0}")]
    Database(#[from] DatabaseError),

    #[error("{0}")]
    Cache(#[from] CacheError),
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
            Error::Parse(e) => HttpResponse::BadRequest().json(json!(self)),
            _ => unimplemented!()
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
            _ => unimplemented!()
        }
    }
}