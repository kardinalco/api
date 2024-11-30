use crate::exceptions::error::Error;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

pub struct CredentialsRoute;

impl CredentialsRoute {
    async fn list_credentials() -> Result<String, Error> {
        Ok("".to_string())
    }

    async fn get_credentials() -> Result<String, Error> {
        Ok("".to_string())
    }

    async fn create_credentials() -> Result<String, Error> {
        Ok("".to_string())
    }

    async fn update_credentials() -> Result<String, Error> {
        Ok("".to_string())
    }

    async fn delete_credentials() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for CredentialsRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/credentials")
                .route("", get().to(Self::list_credentials))
                .route("", post().to(Self::create_credentials))
                .route("{credentials_id}", get().to(Self::get_credentials))
                .route("{credentials_id}", put().to(Self::update_credentials))
                .route("{credentials_id}", delete().to(Self::delete_credentials))
        );
            
    }
}
