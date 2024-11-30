use crate::api::house::response::HouseType;
use cuid2::{cuid, slug};
use entity::house::ActiveModel;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct HouseCreateRequest {
    pub name: String,
    pub address: String,
    pub city: String,
    pub description: Option<String>,
}

impl HouseCreateRequest {
    pub fn into_model(self) -> ActiveModel {
        ActiveModel {
            id: Set(cuid()),
            name: Set(self.name),
            code: Set(slug()),
            description: Set(self.description),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize, Validate, Serialize)]
pub struct HouseUpdateRequest {
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: Option<String>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub zip_code: Option<String>,
    pub location_type: Option<HouseType>,
    pub owner_name: Option<String>,
    pub owner_contact_info: Option<String>,
    pub owner_phone: Option<String>,
    pub owner_email: Option<String>,
    pub built_year: Option<i32>,
    pub acquired_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct HouseInviteUserRequest {
    pub users: Vec<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct HouseRevokeUserRequest {
    pub users: Vec<String>,
}
