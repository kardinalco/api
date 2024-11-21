use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use entity::prelude::User;
use entity::user::Column::{DeletedBy, Email};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::api::auth::request::{AuthLoginRequest, AuthRegisterRequest};
use crate::api::auth::response::{AuthLoginResponse, AuthRegisterResponse};
use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;

pub struct AuthDomain;

impl AuthDomain {
    pub async fn login(body: AuthLoginRequest, db: DatabaseConnection, session: Session) -> Result<AuthLoginResponse, Error> {
        let user = User::find()
            .filter(Email.eq(body.email.clone()))
            .filter(DeletedBy.is_null())
            .one(&db)
            .await?.ok_or(AuthenticateError::WrongCredentials)?;
        if !bcrypt::verify(&body.password, user.password.as_str())? {
            return Err(AuthenticateError::WrongCredentials.into());
        }
        session.insert("user_id", user.id.clone())?;
        Ok(AuthLoginResponse::new(user))
    }

    pub async fn register(body: AuthRegisterRequest, db: DatabaseConnection) -> Result<AuthRegisterResponse, Error> {
        let user = User::find()
            .filter(Email.eq(body.email.clone()))
            .one(&db)
            .await?;
        if let Some(_) = user {
            return Err(AuthenticateError::UserAlreadyRegistered.into());
        }
        body.hash_password()?.into_model().insert(&db).await?;
        Ok(AuthRegisterResponse { message: "User registered successfully" })
    }

    pub async fn logout(auth_session: AuthSession) -> Result<HttpResponse, Error> {
        auth_session.session.remove("user_id");
        Ok(HttpResponse::new(StatusCode::OK))
    }
}