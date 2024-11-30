use actix_web::ResponseError;
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum DatabaseError {
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

impl ResponseError for DatabaseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().finish()
    }
}

impl From<sea_orm::error::DbErr> for DatabaseError {
    fn from(value: sea_orm::error::DbErr) -> Self {
        println!("{:?}", value);
        match value {
            _ => Self::ConnectionError(String::from("TODO")),
        }
    }
}
