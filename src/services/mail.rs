use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use serde_json::json;
use settings::mail::{ListMonk, Mail};
use crate::exceptions::error::Error;
use crate::services::cache::CachedSettings;
use crate::utils::listmonk::Client;
use crate::utils::settings::Settings;

pub struct MailService<'a> {
    db: &'a DatabaseConnection, 
    cache: &'a Pool<RedisConnectionManager>
}

#[derive(Serialize)]
pub struct MailVerification {
    pub code: String,
}

pub struct MailWelcome {
    pub firstname: String,
    pub lastname: String
}

#[derive(Serialize)]
pub struct PasswordChanged {
    pub firstname: String,
    pub lastname: String,
    pub date: chrono::NaiveDateTime
}

impl<'a> MailService<'a> {
    
    pub fn new(db: &'a DatabaseConnection, cache: &'a Pool<RedisConnectionManager>) -> Self {
        MailService { db, cache }
    }

    async fn get_client(&self) -> Result<(Client, ListMonk), Error> {
        let mail_settings = Settings::<Mail>::new(&self.cache, &self.db)
            .await?
            .into_inner();
        let list_monk = mail_settings.listmonk();
        Ok((Client::new(list_monk.api_host.clone(), list_monk.api_user.clone(), list_monk.api_key.clone()), list_monk.clone()))
    }
    
    pub async fn create_subscriber(&self, email: &str, name: &str) -> Result<i32, Error> {
        let (client, _) = self.get_client().await?;
        Ok(client.create_subscriber(email, name).await?.data.id)
    }
    
    pub async fn send_verification_mail(&self, id: i32, body: &MailVerification) -> Result<(), Error> {
        let (client, listmonk) = self.get_client().await?;
        client.send_transactional_mail(id, listmonk.templates.verification, body).await
    }
    
    pub async fn send_welcome_mail(&self, email: &str, body: &MailWelcome) -> Result<(), Error> {
        let (client, listmonk) = self.get_client().await?;
        client.send_transactional_mail_with_email(email, listmonk.templates.welcome, &json!({"firstname": body.firstname, "lastname": body.lastname})).await
    }
    
    pub async fn send_reset_password_mail(&self, email: &str, body: &MailVerification) -> Result<(), Error> {
        let (client, listmonk) = self.get_client().await?;
        client.send_transactional_mail_with_email(email, listmonk.templates.reset_password, &body).await
    }
    
    pub async fn send_password_changed_mail(&self, email: &str, body: &PasswordChanged) -> Result<(), Error> {
        let (client, listmonk) = self.get_client().await?;
        client.send_transactional_mail_with_email(&email, listmonk.templates.password_changed, &body).await
    }
}