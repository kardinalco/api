use entity::credentials::ActiveModel;
use entity::sea_orm_active_enums::CredentialsType;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, Deserialize, Validate)]
pub struct CredentialCreate {
    pub house_id: String,
    pub name: String,
    pub username: Option<String>,
    pub password: Vec<String>,
    pub description: Option<String>,
    pub r#type: Type,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CredentialUpdate {
    pub name: String,
    pub username: Option<String>,
    pub password: Option<Vec<String>>,
    pub description: Option<String>,
    pub r#type: Type,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Type {
    Wifi,
    Email,
    CloudStorage,
    Application,
    Device,
    Database,
    StreamingService,
    BankAccount,
    Website,
    Other,
}

impl Into<CredentialsType> for Type {
    fn into(self) -> CredentialsType {
        match self {
            Type::Wifi => CredentialsType::Wifi,
            Type::Email => CredentialsType::Email,
            Type::CloudStorage => CredentialsType::CloudStorage,
            Type::Application => CredentialsType::Application,
            Type::Device => CredentialsType::Device,
            Type::Database => CredentialsType::Database,
            Type::StreamingService => CredentialsType::StreamingService,
            Type::BankAccount => CredentialsType::BankAccount,
            Type::Website => CredentialsType::Website,
            Type::Other => CredentialsType::Other,
        }
    }
}

impl From<CredentialsType> for Type {
    fn from(value: CredentialsType) -> Self {
        match value {
            CredentialsType::Wifi => Type::Wifi,
            CredentialsType::Email => Type::Email,
            CredentialsType::CloudStorage => Type::CloudStorage,
            CredentialsType::Application => Type::Application,
            CredentialsType::Device => Type::Device,
            CredentialsType::Database => Type::Database,
            CredentialsType::StreamingService => Type::StreamingService,
            CredentialsType::BankAccount => Type::BankAccount,
            CredentialsType::Website => Type::Website,
            CredentialsType::Other => Type::Other,
        }
    }
}

impl CredentialCreate {
    pub fn into_model(self, created_by: &str) -> ActiveModel {
        ActiveModel {
            id: Set(cuid2::cuid()),
            username: Set(self.username),
            password: Set(self.password),
            description: Set(self.description),
            credentials_type: Set(self.r#type.into()),
            created_by: Set(Some(created_by.to_owned())),
            name: Set(self.name),
            ..Default::default()
        }
    }
}

impl CredentialUpdate {
    pub fn into_model(self, updated_by: &str) -> ActiveModel {
        ActiveModel {
            username: Set(self.username),
            description: Set(self.description),
            password: self.password.map_or(Default::default(), |address| Set(address)),
            credentials_type: Set(self.r#type.into()),
            updated_by: Set(Some(updated_by.to_owned())),
            updated_at: Set(Some(chrono::Local::now().naive_local())),
            ..Default::default()
        }
    }
}