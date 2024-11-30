use actix_web::web::{delete, get, post, put, scope, ServiceConfig};
use crate::exceptions::error::Error;
use crate::utils::route::Route;

pub struct SettingsRoute;

impl SettingsRoute {
    pub async fn list_settings() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn get_settings() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn create_settings() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_settings() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_settings() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for SettingsRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/settings")
                .route("", get().to(Self::list_settings))
                .route("", post().to(Self::create_settings))
                .route("/{id}", get().to(Self::get_settings))
                .route("/{id}", put().to(Self::update_settings))
                .route("/{id}", delete().to(Self::delete_settings))
        );
    }
}