use crate::api::auth::request::{AuthLoginRequest, AuthRegisterRequest};
use crate::domain::role::RoleDomain;
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
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use settings::google::Google;
use tracing::instrument;

pub struct AuthDomain;

impl AuthDomain {
    #[instrument(skip(body, db, session))]
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

    pub fn insert_session(session: &Session, user: entity::user::Model, password: String) -> Result<(), Error> {
        if !bcrypt::verify(&password, user.password.as_str())? {
            return Err(AuthenticateError::WrongCredentials.into());
        }
        session.insert("user_id", user.id.clone())?;
        Ok(())
    }

    pub async fn register(body: AuthRegisterRequest, db: DatabaseConnection, cache: Pool<RedisConnectionManager>) -> Result<(), Error> {
        let user = User::find()
            .filter(Email.eq(body.email.clone()))
            .one(&db)
            .await?;
        if let Some(_) = user {
            return Err(AuthenticateError::UserAlreadyRegistered.into());
        }
        let user = body.hash_password()?.into_model().insert(&db).await?;
        RoleDomain::add_user_to_default_role(&db, &cache, &user).await?;
        Ok(())
    }

    #[instrument(skip(auth_session), fields(user_id = %auth_session.user.id))]
    pub async fn logout(auth_session: AuthSession) -> Result<(), Error> {
        auth_session.session.remove("user_id");
        Ok(())
    }

    #[instrument(skip(db, cache))]
    pub async fn build_google_auth_url(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>) -> Result<String, Error> {
        let google_auth = Settings::<Google>::new(cache, db).await?.into_inner();
        if !google_auth.is_enabled() {
            return Err(AuthenticateError::ThirdPartyNotEnabled("Google Auth").into());
        }
        Ok(google_auth.build_authorize_url())
    }

    #[instrument(skip(db, cache, session))]
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
