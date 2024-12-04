use entity::permission::Model;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Permission {
    pub id: String,
    pub description: Option<String>,
    pub action: String,
    pub resource: String,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Model> for Permission {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            description: value.description,
            action: value.action,
            resource: value.resource,
            created_at: value.created_at,
            created_by: None,
            updated_by: None,
            updated_at: None,
        }
    }
}