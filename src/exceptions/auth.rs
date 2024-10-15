use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum AuthenticateError {
    #[error("Wrong authentication credentials")]
    WrongCredentials,

    #[error("Invalid reset password token")]
    InvalidResetPasswordToken,
}

impl ResponseError for AuthenticateError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AuthenticateError::WrongCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthenticateError::InvalidResetPasswordToken => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .json(self.to_string())
            .into()
    }
}

