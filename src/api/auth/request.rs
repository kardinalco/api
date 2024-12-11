use crate::exceptions::error::Error;
use crate::services::hash::hash;
use cuid2::cuid;
use entity::user;
use sea_orm::ActiveValue::Set;
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
pub struct AuthLoginWithGoogleRequest {
    #[validate(length(min = 8, max = 255))]
    pub code: String,
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

impl AuthRegisterRequest {
    pub fn into_model(self) -> user::ActiveModel {
        user::ActiveModel {
            id: Set(cuid()),
            email: Set(self.email),
            first_name: Set(self.firstname),
            last_name: Set(self.lastname),
            password: Set(self.password),
            ..Default::default()
        }
    }

    pub fn hash_password(self) -> Result<Self, Error> {
        Ok(Self {
            password: hash(&self.password)?,
            ..self
        })
    }
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthVerifyRequest {
    pub code: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthResetPasswordRequest {
    pub code: String,
    pub password: String,
}