use actix_multipart::form::MultipartForm;
use crate::api::user::request::{UploadProfilePictureRequest, UserDeleteRequest, UserUpdateRequest};
use crate::api::user::response::{UserListResponse, UserResponse};
use crate::domain::user::UserDomain;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, Path};
use tracing::instrument;
use permission::resource::Resource;
use permission::user::UserPermission;
use crate::entity::user::UserFields;
use crate::extractors::filter::Filter;
use crate::extractors::pagination::Pagination;
use crate::utils::response::ResponseStruct;

pub struct UserRoute;

type User = ResponseStruct<UserResponse>;
type Users = ResponseStruct<UserListResponse>;

impl UserRoute {

    #[instrument(name = "handler::get")]
    pub async fn get(session: AuthSession, user_id: Path<String>) -> User {
        session.enforce_or(vec![Resource::User(UserPermission::Read), Resource::User(UserPermission::ReadSelf)]).await?;
        let user = UserDomain::find_active_user_by_id(&session, &user_id.into_inner()).await?;
        Ok(UserResponse::new(user))
    }

    #[instrument(name = "handler::update")]
    pub async fn update(session: AuthSession, user_id: Path<String>, body: Dto<UserUpdateRequest>) -> User {
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        let user = UserDomain::update(&session, user_id.into_inner(), body.into_inner()).await?;
        Ok(UserResponse::new(user))
    }

    #[instrument(name = "handler::delete")]
    pub async fn delete(session: AuthSession, path: Path<String>) -> User {
        session.enforce_or(vec![Resource::User(UserPermission::Delete), Resource::User(UserPermission::DeleteSelf)]).await?;
        let user = UserDomain::delete(session, &path.into_inner()).await?;
        Ok(UserResponse::new(user))
    }
    
    #[instrument(name = "handler::delete_many")]
    pub async fn delete_many(session: AuthSession, body: Dto<UserDeleteRequest>) -> Users {
        session.enforce(Resource::User(UserPermission::Delete)).await?;
        let users = UserDomain::delete_many(session, body.into_inner().users).await?;
        Ok(UserListResponse::new(users))
    }

    #[instrument(name = "user::handler::query")]
    pub async fn query(session: AuthSession, filter: Filter<UserFields>, pag: Pagination) -> Users {
        session.enforce_or(vec![Resource::User(UserPermission::Read), Resource::User(UserPermission::ReadSelf)]).await?;
        let users = UserDomain::query(&session, filter, pag).await?;
        Ok(UserListResponse::new(users))
    }

    #[instrument(name = "handler::update_user_profile_picture", skip(body))]
    pub async fn update_picture(session: AuthSession, path: Path<String>, body: MultipartForm<UploadProfilePictureRequest>) -> User {
        body.verify_content_type()?;
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        let user = UserDomain::update_profile_picture(session, path.as_str(), body.into_inner()).await?;
        Ok(UserResponse::new(user))
    }

    #[instrument(name = "handler::delete_profile_picture")]
    pub async fn delete_picture(session: AuthSession, path: Path<String>) -> User {
        session.enforce_or(vec![Resource::User(UserPermission::Update), Resource::User(UserPermission::UpdateSelf)]).await?;
        let user = UserDomain::delete_profile_picture(session, path.as_str()).await?;
        Ok(UserResponse::new(user))
    }
}

impl Route for UserRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/user")
                .route("", delete().to(UserRoute::delete_many))
                .route("query", post().to(UserRoute::query))
                .route("{id}", put().to(UserRoute::update))
                .route("{id}", get().to(UserRoute::get))
                .route("{id}", delete().to(UserRoute::delete))
                .route("{id}/profile", post().to(UserRoute::update_picture))
                .route("{id}/profile", delete().to(UserRoute::delete_picture))
        );
    }
}
