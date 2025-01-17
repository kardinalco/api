use actix_web::http::StatusCode;
use actix_web::ResponseError;
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug, Serialize)]
pub enum RequestError {
    #[error("Google invalid state")]
    GoogleInvalidState,
    #[error("{0}")]
    Internal(String),
    #[error("Invalid mime type !")]
    InvalidMimeType
}

impl ResponseError for RequestError {
    fn status_code(&self) -> StatusCode {
        match self {
            RequestError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RequestError::GoogleInvalidState => StatusCode::BAD_REQUEST,
            RequestError::InvalidMimeType => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(json!({
            "message": self.to_string()
        }))
    }
}

impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> Self {
        error!("ReqwestError: {:?}", err);
        RequestError::Internal(err.to_string())
    }
}
