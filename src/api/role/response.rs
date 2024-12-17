use serde::{Serialize};
use crate::api::permission::response::Permission;

#[derive(Serialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<Permission>,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Role {
    pub fn from_model(value: (entity::role::Model, Vec<entity::permission::Model>)) -> Self {
        let (role, permissions) = value;
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            permissions: permissions.into_iter().map(Permission::from).collect(),
            created_at: role.created_at,
            created_by: role.created_by,
            updated_by: role.updated_by,
            updated_at: role.updated_at,
        }
    }
}