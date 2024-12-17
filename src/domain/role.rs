use crate::entity::entity::{Create, SoftDelete, Update};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::services::cache::CachedSettings;
use crate::services::entity::{RoleCreatedByUser, WithCreatedByUser, WithPagination};
use crate::utils::settings::Settings;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use entity::{permission, role, user};
use entity::role::Model;
use entity::role_permission::ActiveModel;
use sea_orm::sea_query::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, QuerySelect, TransactionTrait};
use settings::global::Global;
use tracing::instrument;
use crate::api::role::request::{CreateRoleBody, UpdateRole};
use crate::extractors::auth_session::AuthSession;
use crate::extractors::pagination::Pagination;

pub struct RoleDomain;

impl RoleDomain {
    #[instrument]
    pub async fn list(db: &DatabaseConnection, pag: Pagination) -> Result<Vec<(Model, Vec<permission::Model>)>, Error> {
        Ok(role::Entity::find()
            .find_with_related(permission::Entity)
            .with_pagination(pag)
            .all(db)
            .await?)
    }

    #[instrument]
    pub async fn find_role_by_id(db: &DatabaseConnection, id: &str) -> Result<Model, Error> {
        let a = role::Entity::find_by_id(id)
            .find_with_related(user::Entity)
            .all(db)
            .await?;
        println!("{:?}", a);
        todo!()
    }

    #[instrument]
    async fn _find_role_by_name(db: &DatabaseConnection, name: &str) -> Result<Model, Error> {
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

    #[instrument]
    pub async fn get_roles_and_permissions(db: &DatabaseConnection) -> Result<Vec<(Model, Vec<permission::Model>)>, Error> {
        Ok(role::Entity::find()
            .find_with_related(permission::Entity)
            .columns(vec![role::Column::Id, role::Column::Name, role::Column::Description])
            .all(db)
            .await?)
    }

    #[instrument]
    pub async fn get_user_role(db: &DatabaseConnection, user: &user::Model) -> Result<Model, Error> {
        let role = user
            .find_related(role::Entity)
            .columns(vec![role::Column::Id, role::Column::Name, role::Column::Description])
            .one(db).await?;
        match role {
            Some(role) => Ok(role),
            None => Err(Error::Entity(EntityError::NotFound("UserRole", user.id.to_owned()))),
        }
    }

    #[instrument]
    pub async fn add_user_to_default_role(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, user: &user::Model) -> Result<(), Error> {
        let settings = Settings::<Global>::new(cache, db).await?.into_inner();
        let role = Self::_find_role_by_name(db, settings.get_default_role_name()).await?;
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

    #[instrument]
    pub async fn create(session: AuthSession, body: CreateRoleBody) -> Result<(Model, Vec<permission::Model>), Error> {
        let tx = session.db.begin().await?;
        let role = role::Entity::create(&tx, body.clone(), Some(session.user.id)).await?;
        let permissions = permission::Entity::find()
            .filter(permission::Column::Id.is_in(body.permissions.clone()))
            .all(&tx)
            .await?;
        entity::role_permission::Entity::insert_many(body.permissions.into_iter().map(|x| {
            ActiveModel {
                role_id: Set(role.id.clone()),
                permission_id: Set(x),
                created_at: Set(chrono::Utc::now().naive_utc()),
            }
        }));
        tx.commit().await?;
        Ok((role, permissions))
    }

    #[instrument]
    pub async fn delete(session: &AuthSession, id: &str) -> Result<Model, Error> {
        let role = Self::find_role_by_id(&session.db, id).await?;
        Ok(role.soft_delete(&session.db, Some(session.user.id.clone())).await?)
    }

    #[instrument]
    pub async fn update(session: AuthSession, id: &str, body: UpdateRole) -> Result<Model, Error> {
        let role = Self::find_role_by_id(&session.db, id).await?;
        Ok(role.update(&session.db, body, Some(session.user.id.clone())).await?)
    }

    #[instrument]
    pub async fn get(db: &DatabaseConnection, id: &str) -> Result<(Model, Vec<permission::Model>), Error> {
        let roles = role::Entity::find_by_id(id)
            .filter(role::Column::Id.eq(id))
            .find_with_related(permission::Entity)
            .all(db)
            .await?;
        roles.get(0)
            .map(|x| Ok(x.clone()))
            .unwrap_or_else(|| Err(Error::Entity(EntityError::NotFound("Role", id.to_owned()))))
    }
    
}