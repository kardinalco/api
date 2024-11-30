use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,

    #[error("Email already used")]
    UserAlreadyRegistered,

    #[error("Invalid session, try to login before")]
    NeedSession,

    #[error("Cannot create user session, wait a moment and try again !")]
    CannotCreateUserSession,

    #[error("'{0}' third party is not enabled")]
    ThirdPartyNotEnabled(&'static str),

    #[error("{0}")]
    Unauthorized(String),
}

impl ResponseError for AuthenticateError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticateError::WrongCredentials => StatusCode::UNAUTHORIZED,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(json!({"message": self.to_string()}))
            .into()
    }
}
