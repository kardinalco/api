use actix_web::Responder;
use actix_web::web::{get, post, put, scope, delete, Path};
use tracing::instrument;
use crate::api::user::request::UserUpdateRequest;
use crate::api::user::response::{UserListResponse, UserResponse};
use crate::domain::user::UserDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;

pub struct UserRoute;

impl UserRoute {

    #[instrument(skip(session, db))] 
    pub async fn get_user(session: AuthSession, db: DbReq, user_id: Path<String>) -> Result<UserResponse, Error> {
        let user = if session.user.id.eq_ignore_ascii_case(user_id.as_str()) {
            UserDomain::find_active_user_by_id(&user_id.into_inner(), &db.into_inner()).await?
        } else {
            //TODO check user permission !
            UserDomain::find_active_user_by_id(&user_id.into_inner(), &db.into_inner()).await?
        };
        Ok(UserResponse::new(user))
    }

    #[instrument(skip(session, db))]
    pub async fn update_user(session: AuthSession, db: DbReq, user_id: Path<String>, body: Dto<UserUpdateRequest>) -> Result<UserResponse, Error> {
        let user = if session.user.id.eq_ignore_ascii_case(user_id.as_str()) {
            UserDomain::update(session.user.id, user_id.into_inner(), &db.into_inner(), body.into_inner()).await?
        } else {
            //TODO check user permission !
            UserDomain::update(session.user.id, user_id.into_inner(), &db.into_inner(), body.into_inner()).await?
        };
        Ok(UserResponse::new(user))
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
        let _users = UserDomain::list_user(session, db.into_inner()).await?;
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
                .route("{id}", put().to(UserRoute::update_user))
                .route("{id}", get().to(UserRoute::get_user))
                .route("{id}", delete().to(UserRoute::delete_user))
                .route("{id}/upload-picture", post().to(UserRoute::update_user_profile_picture))
                .route("{id}/upload-picture", delete().to(UserRoute::delete_user_profile_picture))
        );
    }
}