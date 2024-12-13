use entity::prelude::User;
use crate::api::user::request::{UploadProfilePictureRequest, UserUpdateRequest};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use entity::user::{Entity, Model};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_query::Expr;
use tracing::instrument;
use permission::resource::Resource;
use permission::user::UserPermission;
use crate::entity::user::{DeleteUser, UpdateUser, UserFields};
use crate::extractors::auth_session::AuthSession;
use crate::extractors::filter::{Filter};
use crate::extractors::pagination::Pagination;
use crate::services::entity::{WithFilter, WithPagination};
use crate::services::storage::StorageService;

pub struct UserDomain;

impl UserDomain {

    pub async fn find_active_user_by_id(session: &AuthSession, user_id: &str) -> Result<Model, Error> {
        match session.user.id == user_id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Read)).await?
        };
        Entity::find_by_id(user_id)
            .one(&session.db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("User", user_id.to_owned())))
    }

    #[instrument(name = "domain::update")]
    pub async fn update(session: &AuthSession, user_id: String, body: UserUpdateRequest) -> Result<Model, Error> {
        match user_id == session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Update)).await?
        };
        let user = Entity::find_by_id(user_id.clone())
            .one(&session.db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("User", user_id)))?;
        user.update(&session.db, body, Some(session.user.id.clone())).await
    }

    #[instrument(name = "domain::update_profile_picture")]
    pub async fn update_profile_picture(session: AuthSession, user_id: &str, picture: UploadProfilePictureRequest) -> Result<Model, Error> {
        match user_id == &session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Update)).await?
        };
        let user = Self::find_active_user_by_id(&session, user_id).await?;
        
        let code = StorageService::upload_user_profile_picture(&session.db, &session.cache, picture.file).await?;
        user.update_profile_picture(&session.db, &code, Some(session.user.id)).await
    }

    #[instrument(name = "domain::delete_profile_picture")]
    pub async fn delete_profile_picture(session: AuthSession, user_id: &str) -> Result<Model, Error> {
        match user_id == session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Update)).await?
        };
        let user = Self::find_active_user_by_id(&session, user_id).await?;
        match &user.picture.clone() {
            Some(picture) => {
                StorageService::delete_user_profile_picture(&session.db, &session.cache, &picture).await?;
                user.update_profile_picture(&session.db, picture, Some(session.user.id)).await
            },
            None => Ok(user)
        }
    }

    #[instrument(name = "domain::delete")]
    pub async fn delete(session: AuthSession, user_id: &str) -> Result<Model, Error> {
        match user_id == session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Delete)).await?
        };
        let user = Self::find_active_user_by_id(&session, user_id).await?;
        Ok(user.delete_user(&session.db, Some(session.user.id)).await?)
    }

    #[instrument(name = "domain::delete_many")]
    pub async fn delete_many(session: AuthSession, body: Vec<String>) -> Result<Vec<Model>, Error> {
        Ok(User::update_many()
            .filter(entity::user::Column::Id.is_in(body))
            .col_expr(entity::user::Column::DeletedBy, Expr::value(session.user.id))
            .col_expr(entity::user::Column::DeletedAt, Expr::value(chrono::Utc::now().naive_utc()))
            .col_expr(entity::user::Column::IsDeleted, Expr::value(true))
            .exec_with_returning(&session.db)
            .await?)
    }

    #[instrument(name = "user::domain::query", skip(session))]
    pub async fn query(session: &AuthSession, filter: Filter<UserFields>, pagination: Pagination) -> Result<Vec<Model>, Error> {
        Ok(Entity::find()
            .with_filter(filter)
            .with_pagination(pagination)
            .all(&session.db).await?)
    }
    
}