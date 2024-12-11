use actix_multipart::form::MultipartForm;
use crate::api::user::request::{UploadProfilePictureRequest, UserUpdateRequest};
use crate::api::user::response::{UserListResponse, UserResponse};
use crate::domain::user::UserDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, Path};
use actix_web::{HttpResponse, Responder};
use tracing::instrument;
use permission::resource::Resource;
use permission::user::UserPermission;

pub struct UserRoute;

impl UserRoute {
    #[instrument(skip(session))]
    pub async fn get_user(session: AuthSession, user_id: Path<String>) -> Result<UserResponse, Error> {
        session.enforce_or(vec![Resource::User(UserPermission::Read), Resource::User(UserPermission::ReadSelf)]).await?;
        let user = UserDomain::find_active_user_by_id(&session, &user_id.into_inner()).await?;
        Ok(UserResponse::new(user))
    }

    #[instrument(skip(session, db))]
    pub async fn update_user(session: AuthSession, db: DbReq, user_id: Path<String>, body: Dto<UserUpdateRequest>) -> Result<UserResponse, Error> {
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        let user = UserDomain::update(session, user_id.into_inner(), &db.into_inner(), body.into_inner()).await?;
        Ok(UserResponse::new(user))
    }

    #[instrument(skip(session))]
    pub async fn delete_user(session: AuthSession, path: Path<String>) -> impl Responder {
        ""
    }

    #[instrument(skip(_session))]
    pub async fn list_user(_session: AuthSession, _db: DbReq) -> Result<UserListResponse, Error> {
        //let _users = UserDomain::list_user(session, db.into_inner()).await?;
        todo!()
    }

    #[instrument(skip(session, body))]
    pub async fn update_user_profile_picture(session: AuthSession, path: Path<String>, body: MultipartForm<UploadProfilePictureRequest>) -> Result<HttpResponse, Error> {
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        body.verify_content_type()?;
        UserDomain::update_user_profile_picture(session, path.as_str(), body.into_inner()).await?;
        Ok(HttpResponse::Ok().finish())
    }

    #[instrument(skip(session))]
    pub async fn delete_user_profile_picture(session: AuthSession, path: Path<String>) -> Result<HttpResponse, Error> {
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        UserDomain::delete_user_profile_picture(session, path.as_str()).await?;
        Ok(HttpResponse::Ok().finish())
    }
}

impl Route for UserRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/user")
                .route("", get().to(UserRoute::list_user))
                .route("{id}", put().to(UserRoute::update_user))
                .route("{id}", get().to(UserRoute::get_user))
                .route("{id}", delete().to(UserRoute::delete_user))
                .route("{id}/profile", post().to(UserRoute::update_user_profile_picture))
                .route("{id}/profile", delete().to(UserRoute::delete_user_profile_picture))
        );
    }
}
