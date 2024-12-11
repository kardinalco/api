use crate::domain::role::RoleDomain;
use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::cache::CacheError;
use crate::exceptions::error::Error;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use redis::{AsyncCommands, JsonAsyncCommands};
use serde::{Deserialize, Serialize};
use settings::cache::Cache;
use crate::services::cache::CachedSettings;
use crate::utils::settings::Settings;

#[derive(Clone)]
pub struct Permission {
    pub db: Arc<DatabaseConnection>,
    pub cache: Arc<Pool<RedisConnectionManager>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub permissions: Vec<Perm>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perm {
    pub resource: String,
    pub action: String,
}

const KEY: &str = "roles";

impl Permission {
    pub fn new(db: Arc<DatabaseConnection>, cache: Arc<Pool<RedisConnectionManager>>) -> Self {
        Self { db, cache }
    }

    async fn get(&self) -> Result<Vec<Role>, Error> {
        let mut conn = self.cache.get().await?;
        match conn.exists::<_, bool>(KEY).await? {
            true => match self.get_permission_from_cache().await {
                Ok(roles) => Ok(roles),
                Err(_) => self.get_permission_from_db().await,
            },
            false => {
                let permissions = self.get_permission_from_db().await?;
                self.save_permission_to_cache(permissions.clone()).await?;
                Ok(permissions)
            },
        }
    }

    async fn get_permission_from_db(&self) -> Result<Vec<Role>, Error> {
        Ok(
            RoleDomain::get_roles_and_permissions(&self.db).await?.iter().map(|(r, p)| {
                Role {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    permissions: p.iter().map(|p| Perm {
                        resource: p.resource.clone(),
                        action: p.action.clone(),
                    }).collect(),
                }
            }).collect(),
        )
    }

    async fn get_permission_from_cache(&self) -> Result<Vec<Role>, Error> {
        let mut conn = self.cache.get().await?;
        let roles: String = conn.json_get("roles", ".").await?;
        Ok(serde_json::from_str(&roles).map_err(|_| {
            Error::Cache(CacheError::ConnectionError("roles".to_owned()))
        })?)
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    async fn save_permission_to_cache(&self, role: Vec<Role>) -> Result<(), Error> {
        let cache = Settings::<Cache>::new(&self.cache, &self.db).await?.into_inner();
        let mut conn = self.cache.get().await?;
        conn.json_set::<_, &str, Vec<Role>, _>(KEY, ".", &role).await?;
        conn.expire::<_, String>(KEY, cache.get_permission_ttl()).await?;
        Ok(())
    }

    pub async fn enforce(&self, (user, obj, action): (&entity::user::Model, &str, &str)) -> Result<(), Error> {
        let roles = self.get().await?;
        let role = RoleDomain::get_user_role(&self.db, user).await?;
        let permissions = self
            .get_role_permissions(&roles, &role)
            .ok_or(self.build_error(user, obj, action))?;
        if let None = permissions.iter().find(|p| p.resource == obj && (p.action == action || p.action == "all")) {
            return Err(self.build_error(user, obj, action));
        }
        Ok(())
    }

    pub fn get_role_permissions(&self,roles: &Vec<Role>, role: &entity::role::Model) -> Option<Vec<Perm>> {
        roles.iter().find(|r| r.id == role.id).map(|r| r.permissions.clone())
    }

    fn build_error(&self, _user: &entity::user::Model, obj: &str, action: &str) -> Error {
        Error::Auth(AuthenticateError::Unauthorized(format!(
            "required {}.{} to perform that !",
            obj, action
        )))
    }
}
