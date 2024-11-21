use entity::{permission,};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;

pub struct PermissionDomain;

type Permission = permission::Model;

impl PermissionDomain {
        
    pub async fn list_all_permissions(db: &DatabaseConnection) -> Result<Vec<Permission>, Error> {
        Ok(permission::Entity::find().all(db).await?)
    } 
    
    pub async fn list_permission_for_resource(db: &DatabaseConnection, resource: &str) -> Result<Vec<Permission>, Error> {
        Ok(permission::Entity::find()
            .filter(permission::Column::Resource.eq(resource))
            .all(db)
            .await?)
    }
    
    pub async fn find_permission_by_id(db: &DatabaseConnection, id: &str) -> Result<Permission, Error> {
        Ok(permission::Entity::find_by_id(id).one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("".to_string())))?)
    }
    
    pub async fn find_permission_by_name(db: &DatabaseConnection, action: &str, resource: &str) -> Result<Permission, Error> {
        Ok(permission::Entity::find()
            .filter(permission::Column::Action.eq(action))
            .filter(permission::Column::Resource.eq(resource))
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("".to_string())))?)
    }
}