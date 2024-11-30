use actix_web::ResponseError;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum EntityError {
    #[error("Entity '{0}' with identifier '{1}' not found")]
    NotFound(&'static str, String),
    #[error("Invalid data: {0}")]
    NeedToBeOwner(String),
    #[error("Invalid data: {0}")]
    NoPermission(String),
}

impl ResponseError for EntityError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EntityError::NotFound(_, _) => actix_web::http::StatusCode::NOT_FOUND,
            EntityError::NeedToBeOwner(_) => actix_web::http::StatusCode::FORBIDDEN,
            EntityError::NoPermission(_) => actix_web::http::StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code()).json(json!({
            "message": self.to_string(),
        }))
    }
}
