use entity::{permission, role, user};
use sea_orm::{DatabaseConnection, EntityTrait, FromQueryResult, ModelTrait, QueryFilter};
use sea_orm::sea_query::{Expr};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::services::entity::{RoleCreatedByUser, WithCreatedByUser};

pub struct RoleDomain;

type Role = role::Model;

#[derive(Debug, FromQueryResult)]
struct B {
    pub created_by_lastname: String,
}

pub trait PermissionTrait {
    async fn list_permissions(self, db: &DatabaseConnection) -> Result<Vec<permission::Model>, Error>;
    async fn list_permissions_action(self, db: &DatabaseConnection, action: &str) -> Result<Vec<permission::Model>, Error>;
}

impl PermissionTrait for role::Model {
    async fn list_permissions(self, db: &DatabaseConnection) -> Result<Vec<permission::Model>, Error> {
        Ok(self.find_related(permission::Entity).all(db).await?)
    }

    async fn list_permissions_action(self, _db: &DatabaseConnection, _action: &str) -> Result<Vec<permission::Model>, Error> {
        todo!()
    }
}

impl RoleDomain {
    pub async fn list_all_roles(db: &DatabaseConnection) -> Result<Vec<Role>, Error> {
        Ok(role::Entity::find().all(db).await?)
    }

    pub async fn find_role_by_id(db: &DatabaseConnection, id: &str) -> Result<Role, Error> {
        let a = role::Entity::find_by_id(id)
            .find_with_related(user::Entity)
            .all(db)
            .await?;
        println!("{:?}", a);
        todo!()
    }

    pub async fn find_role_by_name(db: &DatabaseConnection, name: &str) -> Result<Role, Error> {
        let role = role::Entity::find()
           .filter(Expr::col(role::Column::Name).eq(name))
           .with_created_user::<RoleCreatedByUser>()
           .one(db)
           .await?;
        Ok(role.ok_or(Error::Entity(EntityError::NotFound("".to_string())))?)
    }
    
    pub async fn test(db: &DatabaseConnection) -> Result<(), Error> {
        let role = Self::find_role_by_name(db, "Admin").await?;
        let permissions = role.find_related(permission::Entity).all(db).await?;
        println!("{:?}", permissions);
        Ok(())
    }

    pub async fn get_roles_and_permissions(db: &DatabaseConnection) -> Result<Vec<(role::Model, Vec<permission::Model>)>, Error> {
        Ok(role::Entity::find()
            .find_with_related(permission::Entity)
            .all(db)
            .await?)
    }
}