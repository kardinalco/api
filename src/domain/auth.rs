use crate::api::auth::request::{AuthLoginRequest, AuthRegisterRequest, AuthResetPasswordRequest};
use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::services::cache::CachedSettings;
use crate::services::google::GoogleService;
use crate::utils::settings::Settings;
use actix_session::Session;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use cuid2::cuid;
use entity::prelude::User;
use entity::sea_orm_active_enums::RegisteredWith;
use entity::user::Column::{DeletedBy, Email};
use redis::AsyncCommands;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use settings::google::Google;
use tracing::{info, instrument};
use crate::domain::mail::MailDomain;
use crate::domain::role::RoleDomain;
use crate::entity::entity::Create;
use crate::entity::user::UpdateUser;
use crate::services::hash::{compare, hash};

pub struct AuthDomain;

impl AuthDomain {

    #[instrument(name = "domain::login", skip(body, db, session))]
    pub async fn login(body: AuthLoginRequest, db: DatabaseConnection, session: Session) -> Result<entity::user::Model, Error> {
        let user = User::find()
            .filter(Email.eq(body.email.clone()))
            .filter(entity::user::Column::RegisteredWith.eq(RegisteredWith::Native))
            .filter(DeletedBy.is_null())
            .one(&db)
            .await?
            .ok_or(AuthenticateError::WrongCredentials)?;
        Self::insert_session(&session, user.clone(), body.password)?;
        Ok(user)
    }

    #[instrument(name = "domain::insert_session", skip(session, password))]
    pub fn insert_session(session: &Session, user: entity::user::Model, password: String) -> Result<(), Error> {
        info!("compare user password");
        if !compare(&password, user.password.as_str())? {
            return Err(AuthenticateError::WrongCredentials.into());
        }
        info!("insert user id into session");
        session.insert("user_id", user.id.clone())?;
        Ok(())
    }

    #[instrument(name = "domain::register", skip(body, db, cache))]
    pub async fn register(body: AuthRegisterRequest, db: DatabaseConnection, cache: Pool<RedisConnectionManager>) -> Result<(), Error> {
        let user = User::find()
            .filter(Email.eq(body.email.clone()))
            .one(&db)
            .await?;
        if let Some(_) = user {
            return Err(AuthenticateError::UserAlreadyRegistered.into());
        }
        let user = entity::user::Entity::create(&db, body, None).await?;
        RoleDomain::add_user_to_default_role(&db, &cache, &user).await?;
        let email_id = MailDomain::registered_user(&db, &cache, &user).await?;
        user.update_mail(&db, email_id, None).await?;
        Ok(())
    }

    #[instrument(name = "domain::verify_user", skip(db, cache))]
    pub async fn verify_user(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, code: &str) -> Result<entity::user::Model, Error> {
        let user_id = cache.get().await?.get::<&str, String>(code).await.map_err(|_| Error::Auth(AuthenticateError::InvalidCode))?;
        let user = User::find()
            .filter(entity::user::Column::IsVerified.eq(false))
            .filter(entity::user::Column::Id.eq(user_id))
            .one(db)
            .await?;
        match user {
            Some(user) => {
                let mut model = user.into_active_model();
                model.is_active = Set(true);
                model.is_verified = Set(true);
                let user = model.update(db).await?;
                MailDomain::welcome_user(db, cache, &user).await?;
                Ok(user)
            },
            None => Err(AuthenticateError::InvalidCode.into())
        }
    }

    #[instrument(name = "domain::logout")]
    pub async fn logout(auth_session: AuthSession) -> Result<(), Error> {
        auth_session.session.remove("user_id");
        Ok(())
    }

    #[instrument(name = "domain::build_google_auth_url", skip(db, cache))]
    pub async fn build_google_auth_url(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>) -> Result<String, Error> {
        let google_auth = Settings::<Google>::new(cache, db).await?.into_inner();
        if !google_auth.is_enabled() {
            return Err(AuthenticateError::ThirdPartyNotEnabled("Google Auth").into());
        }
        Ok(google_auth.build_authorize_url())
    }

    #[instrument(name = "domain::forgot_password", skip(db, cache))]
    pub async fn forgot_password(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, email: &str) -> Result<(), Error> {
        let user = User::find().filter(Email.eq(email)).one(db).await?;
        if let Some(user) = user {
            MailDomain::send_reset_password(db, cache, &user).await?;
        }
        Ok(())
    }

    #[instrument(name = "domain::reset_password", skip(db, cache))]
    pub async fn reset_password(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, body: &AuthResetPasswordRequest) -> Result<(), Error> {
        let user_id = cache.get().await?.get::<&str, String>(&body.code).await.map_err(|_| Error::Auth(AuthenticateError::InvalidCode))?;
        let user = User::find().filter(entity::user::Column::Id.eq(user_id)).one(db).await?;
        match user {
            Some(user) => {
                let user = user.update_password(db, &hash(&body.password)?, None).await?; //TODO: add updated_by
                MailDomain::send_password_changed(db, cache, &user).await?;
                Ok(())
            },
            None => Err(AuthenticateError::InvalidCode.into())
        }
    }

    #[instrument(name = "domain::login_with_google", skip(db, cache, session))]
    pub async fn login_with_google(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, session: &Session, code: &str) -> Result<entity::user::Model, Error> {
        let google_auth = Settings::<Google>::new(cache, db).await?.into_inner();
        if !google_auth.is_enabled() {
            return Err(AuthenticateError::ThirdPartyNotEnabled("Google Auth").into());
        }
        let r = GoogleService::login(&google_auth, code).await?;
        let info = GoogleService::get_user(&google_auth, &r.access_token).await?;
        let user = if let Some(user) = User::find().filter(Email.eq(info.clone().email)).one(db).await? {
            user
        } else {
            let user = entity::user::ActiveModel {
                id: Set(cuid()),
                registered_with: Set(RegisteredWith::Google),
                email: Set(info.email),
                is_active: Set(true),
                is_deleted: Set(false),
                password: Set(cuid()),
                first_name: Set(info.given_name),
                last_name: Set(info.family_name.unwrap_or(".....".to_string())),
                ..Default::default()
            }.insert(db).await?;
            RoleDomain::add_user_to_default_role(db, cache, &user).await?;
            user
        };
        session.insert("user_id", user.id.clone())?;
        Ok(user)
    }
}
