use actix_web::Responder;
use actix_web::web::{get, post, put, scope, delete};
use tracing::instrument;
use crate::api::user::response::UserListResponse;
use crate::domain::user::UserDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::db::DbReq;
use crate::utils::route::Route;

pub struct UserRoute;

impl UserRoute {

    #[instrument]
    pub async fn get_user(_: AuthSession) -> impl Responder {
        ""
    }

    #[instrument]
    pub async fn update_user(_: AuthSession) -> impl Responder {
        ""
    }

    #[instrument]
    pub async fn create_user(_: AuthSession) -> impl Responder {
        ""
    }

    #[instrument]
    pub async fn delete_user(_: AuthSession) -> impl Responder {
        ""
    }

    #[instrument(skip(session))]
    pub async fn list_user(session: AuthSession, db: DbReq) -> Result<UserListResponse, Error> {
        UserDomain::list_user(session, db.into_inner()).await?;
        todo!()
    }

    #[instrument]
    pub async fn update_user_profile_picture(_: AuthSession) -> impl Responder {
        ""
    }

    #[instrument]
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