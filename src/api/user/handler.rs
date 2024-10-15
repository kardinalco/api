use actix_web::Responder;
use actix_web::web::{get, post, put, scope, delete};
use crate::utils::route::Route;

pub struct UserRoute;

impl UserRoute {
    
    pub async fn get_user() -> impl Responder {
        //user_repo(unimplemented!()).find_user_by_id();
        ""
    }

    pub async fn update_user() -> impl Responder {
        ""
    }
    
    pub async fn create_user() -> impl Responder {
        ""
    }

    pub async fn delete_user() -> impl Responder {
        ""
    }
    
    pub async fn list_user() -> impl Responder {
        ""
    }
    
    pub async fn update_user_profile_picture() -> impl Responder {
        ""
    }
    
    pub async fn delete_user_profile_picture() -> impl Responder {
        ""
    }
    
}

impl Route for UserRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/user")
                .route("", get().to(UserRoute::list_user))
                .route("", post().to(UserRoute::create_user))
                .route(":id", put().to(UserRoute::update_user))
                .route(":id", get().to(UserRoute::get_user))
                .route(":id", delete().to(UserRoute::delete_user))
                .route(":id/upload-picture", post().to(UserRoute::update_user_profile_picture))
                .route(":id/upload-picture", delete().to(UserRoute::delete_user_profile_picture))
        );
    }
}