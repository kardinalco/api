use crate::exceptions::error::Error;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

pub struct ProviderHandler;

impl ProviderHandler {
    pub async fn get_provider() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn list_provider() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn create_provider() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_provider() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_provider() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for ProviderHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/providers")
                .route("", get().to(Self::list_provider))
                .route("", post().to(Self::create_provider))
                .route("/{id}", get().to(Self::get_provider))
                .route("/{id}", put().to(Self::update_provider))
                .route("/{id}", delete().to(Self::delete_provider)),
        );
    }
}
