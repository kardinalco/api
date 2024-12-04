use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PermissionUpdate {
    pub name: String,
    pub description: String,
}