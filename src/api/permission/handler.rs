use crate::utils::route::Route;
use actix_web::web::{scope, ServiceConfig};
use crate::exceptions::error::Error;

pub struct PermissionRoute;

impl PermissionRoute {
    pub async fn get_permission() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn list_permission() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for PermissionRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/permissions")
                .route("", actix_web::web::get().to(Self::list_permission))
                .route("/{id}", actix_web::web::get().to(Self::get_permission))
        );
    }
}
