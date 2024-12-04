use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use entity::permission;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::instrument;
use crate::extractors::pagination::Pagination;
use crate::services::entity::WithPagination;

pub struct PermissionDomain;

type Permission = permission::Model;

impl PermissionDomain {
    #[instrument(skip(db))]
    pub async fn list_all_permissions(db: &DatabaseConnection) -> Result<Vec<Permission>, Error> {
        Ok(permission::Entity::find().all(db).await?)
    }

    #[instrument(skip(db))]
    pub async fn list_permission_for_resource(db: &DatabaseConnection, resource: &str) -> Result<Vec<Permission>, Error> {
        Ok(permission::Entity::find()
            .filter(permission::Column::Resource.eq(resource))
            .all(db)
            .await?)
    }

    #[instrument(skip(db))]
    pub async fn list(db: &DatabaseConnection, pag: Pagination) -> Result<Vec<Permission>, Error> {
        Ok(permission::Entity::find()
            .with_pagination(pag)
            .all(db)
            .await?)
    }

    #[instrument(skip(db))]
    pub async fn find_permission_by_id(db: &DatabaseConnection, id: &str) -> Result<Permission, Error> {
        Ok(permission::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound(
                "Permission",
                id.to_owned(),
            )))?)
    }

    #[instrument(skip(db))]
    pub async fn find_permission_by_name(db: &DatabaseConnection, action: &str, resource: &str) -> Result<Permission, Error> {
        Ok(permission::Entity::find()
            .filter(permission::Column::Action.eq(action))
            .filter(permission::Column::Resource.eq(resource))
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound(
                "Permission",
                format!("action: {}, resource: {}", action, resource),
            )))?)
    }
}
