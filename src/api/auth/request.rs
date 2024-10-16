use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthLoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 255))]
    pub password: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthRegisterRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5, max = 128))]
    pub password: String,
    #[validate(length(min = 2, max = 64))]
    pub firstname: String,
    #[validate(length(min = 2, max = 64))]
    pub lastname: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthLogoutRequest {
    pub email: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthResetPasswordRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AuthLoginResponse {
    pub token: String,
}