use serde::Serialize;
use crate::api::credentials::request::Type;

#[derive(Debug, Serialize)]
pub struct Credential {
    pub name: String,
    pub description: Option<String>,
    pub username: Option<String>,
    pub password: Vec<String>,
    pub credentials_type: Type,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub deleted_by: Option<String>,
}

impl Credential {
    pub fn from_model(model: entity::credentials::Model) -> Self {
        Self {
            name: model.name,
            description: model.description,
            username: model.username,
            password: model.password,
            credentials_type: model.credentials_type.into(),
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
            deleted_at: model.deleted_at,
            deleted_by: model.deleted_by,
        }
    }
}