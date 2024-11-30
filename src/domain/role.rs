use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::services::cache::CachedSettings;
use crate::services::entity::{RoleCreatedByUser, WithCreatedByUser};
use crate::utils::settings::Settings;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use entity::{permission, role, user};
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use settings::global::Global;
use tracing::instrument;

pub struct RoleDomain;

pub trait PermissionTrait {
    async fn list_permissions(self, db: &DatabaseConnection) -> Result<Vec<permission::Model>, Error>;
    async fn list_permissions_action(self, db: &DatabaseConnection, action: &str,) -> Result<Vec<permission::Model>, Error>;
}

impl PermissionTrait for role::Model {
    #[instrument(skip(db))]
    async fn list_permissions(self, db: &DatabaseConnection) -> Result<Vec<permission::Model>, Error> {
        Ok(self.find_related(permission::Entity).all(db).await?)
    }

    #[instrument(skip(_db))]
    async fn list_permissions_action(self, _db: &DatabaseConnection, _action: &str) -> Result<Vec<permission::Model>, Error> {
        todo!()
    }
}

impl RoleDomain {
    #[instrument(skip(db))]
    pub async fn list_all_roles(db: &DatabaseConnection) -> Result<Vec<role::Model>, Error> {
        Ok(role::Entity::find().all(db).await?)
    }

    #[instrument(skip(db))]
    pub async fn find_role_by_id(db: &DatabaseConnection, id: &str) -> Result<role::Model, Error> {
        let a = role::Entity::find_by_id(id)
            .find_with_related(user::Entity)
            .all(db)
            .await?;
        println!("{:?}", a);
        todo!()
    }

    #[instrument(skip(db))]
    pub async fn find_role_by_name(db: &DatabaseConnection, name: &str) -> Result<role::Model, Error> {
        let role = role::Entity::find()
            .filter(Expr::col(role::Column::Name).eq(name))
            .with_created_user::<RoleCreatedByUser>()
            .one(db)
            .await?;
        Ok(role.ok_or(Error::Entity(EntityError::NotFound(
            "Role",
            name.to_owned(),
        )))?)
    }

    pub async fn get_roles_and_permissions(db: &DatabaseConnection) -> Result<Vec<(role::Model, Vec<permission::Model>)>, Error> {
        Ok(role::Entity::find()
            .find_with_related(permission::Entity)
            .all(db)
            .await?)
    }

    pub async fn get_user_role(db: &DatabaseConnection, user: &user::Model) -> Result<role::Model, Error> {
        let role = user.find_related(role::Entity).one(db).await?;
        match role {
            Some(role) => Ok(role),
            None => Err(Error::Entity(EntityError::NotFound(
                "UserRole",
                user.id.to_owned(),
            ))),
        }
    }

    pub async fn add_user_to_default_role(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &user::Model) -> Result<(), Error> {
        let settings = Settings::<Global>::new(cache, db).await?.into_inner();
        let role = Self::find_role_by_name(db, settings.get_default_role_name()).await?;
        entity::user_role::Entity::insert(entity::user_role::ActiveModel {
            user_id: Set(user.id.clone()),
            role_id: Set(role.id.clone()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        })
        .exec(db)
        .await?;
        Ok(())
    }
}
