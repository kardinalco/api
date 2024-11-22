use actix_web::web::ServiceConfig;
use crate::utils::route::Route;

pub struct PermissionHandler;

impl PermissionHandler {

}

impl Route for PermissionHandler {
    fn route(_cfg: &mut ServiceConfig) {
        todo!()
    }
}