use crate::api::user::request::{UploadProfilePictureRequest, UserUpdateRequest};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use entity::user::{Entity, Model};
use sea_orm::{DatabaseConnection, EntityTrait};
use permission::resource::Resource;
use permission::user::UserPermission;
use crate::entity::user::{UpdateUser, UserFields};
use crate::extractors::auth_session::AuthSession;
use crate::extractors::filter::{Filter};
use crate::services::entity::WithFilter;
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

    pub async fn update(session: AuthSession, user_id: String, db: &DatabaseConnection, body: UserUpdateRequest) -> Result<Model, Error> {
        match user_id == session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Update)).await?
        };
        let user = Entity::find_by_id(user_id.clone())
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("User", user_id)))?;

        user.update(db, body, Some(session.user.id)).await
    }
    
    pub async fn update_user_profile_picture(session: AuthSession, user_id: &str, picture: UploadProfilePictureRequest) -> Result<Model, Error> {
        match user_id == &session.user.id {
            true => (),
            false => session.enforce(Resource::User(UserPermission::Update)).await?
        };
        let user = Self::find_active_user_by_id(&session, user_id).await?;
        
        let code = StorageService::upload_user_profile_picture(&session.db, &session.cache, picture.file).await?;
        user.update_profile_picture(&session.db, &code, Some(session.user.id)).await
    }
    
    pub async fn delete_user_profile_picture(session: AuthSession, user_id: &str) -> Result<Model, Error> {
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

    pub async fn query(session: &AuthSession, filter: Filter<UserFields>) -> Result<Vec<Model>, Error> {
        Ok(Entity::find()
            .with_filter(filter)
            .all(&session.db).await?)
    }
    
}