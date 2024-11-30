use actix_web::web::{delete, get, post, put, scope, ServiceConfig};
use crate::exceptions::error::Error;
use crate::utils::route::Route;

pub struct PetRoute;

impl PetRoute {
    pub async fn list_pet() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn get_pet() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn create_pet() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_pet() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_pet() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for PetRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/pets")
                .route("", get().to(Self::list_pet))
                .route("", post().to(Self::create_pet))
                .route("/{id}", get().to(Self::get_pet))
                .route("/{id}", put().to(Self::update_pet))
                .route("/{id}", delete().to(Self::delete_pet))
        );
    }
}