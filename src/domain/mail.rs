use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use chrono::Utc;
use redis::{AsyncCommands};
use sea_orm::DatabaseConnection;
use tracing::instrument;
use crate::exceptions::error::Error;
use crate::services::hash::hash;
use crate::services::mail::{MailService, MailVerification, MailWelcome, PasswordChanged};

pub struct MailDomain;

impl MailDomain {

    #[instrument(skip(db, cache))]
    pub async fn registered_user(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &entity::user::Model) -> Result<i32, Error> {
        let mail = MailService::new(&db, &cache);
        let email_id = mail.create_subscriber(&user.email, &format!("{} {}", user.first_name, user.last_name)).await?;
        let code = BASE64_STANDARD.encode(hash(&user.id)?);
        let mut pool = cache.get().await?;
        pool.set::<_, String, String>(code.clone(), user.id.clone()).await?;
        pool.expire::<_, String>(code.clone(), 60 * 60 * 24).await?;
        mail.send_verification_mail(email_id, &MailVerification {code}).await?;
        Ok(email_id)
    }

    #[instrument(skip(db, cache))]
    pub async fn welcome_user(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &entity::user::Model) -> Result<(), Error> {
        let mail = MailService::new(&db, &cache);
        mail.send_welcome_mail(&user.email.clone(), &MailWelcome {firstname: user.first_name.clone(), lastname: user.last_name.clone()}).await?;
        Ok(())
    }

    #[instrument(skip(db, cache))]
    pub async fn send_reset_password(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &entity::user::Model) -> Result<(), Error> {
        let mail = MailService::new(&db, &cache);
        let code = BASE64_STANDARD.encode(cuid2::cuid());
        let mut pool = cache.get().await?;
        pool.set::<_, String, String>(code.clone(), user.id.clone()).await?;
        pool.expire::<_, String>(code.clone(), 60 * 60 * 24).await?;
        mail.send_reset_password_mail(&user.email, &MailVerification {code}).await?;
        Ok(())
    }

    #[instrument(skip(db, cache))]
    pub async fn send_password_changed(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &entity::user::Model) -> Result<(), Error> {
        let mail = MailService::new(&db, &cache);
        mail.send_password_changed_mail(&user.email, &PasswordChanged {date: user.updated_at.unwrap_or(Utc::now().naive_utc()), firstname: user.first_name.clone(), lastname: user.last_name.clone()}).await?;
        Ok(())
    }

}