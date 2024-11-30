use crate::api::user::response::User;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, Responder};
use entity::sea_orm_active_enums::HouseLocationType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct HouseCreatedResponse {
    pub message: String,
    pub house: House,
}

impl HouseCreatedResponse {
    pub fn new(house: entity::house::Model) -> Self {
        Self {
            message: "House created successfully".to_string(),
            house: House::from_model(house, None),
        }
    }
}

impl Responder for HouseCreatedResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::build(StatusCode::CREATED).json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct HouseListResponse {
    pub count: usize,
    pub houses: Vec<House>,
}

impl HouseListResponse {
    pub fn new(houses: Vec<entity::house::Model>) -> Self {
        Self {
            count: houses.len(),
            houses: houses
                .into_iter()
                .map(|house| House::from_model(house, None))
                .collect(),
        }
    }
}

impl Responder for HouseListResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct House {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub code: String,
    pub zip_code: Option<String>,
    pub users: Option<Vec<User>>,
    pub location_type: HouseType,
    pub owner_name: Option<String>,
    pub owner_contact_info: Option<String>,
    pub owner_phone: Option<String>,
    pub owner_email: Option<String>,
    pub built_year: Option<i32>,
    pub acquired_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HouseType {
    Owned,
    Rented,
    Unknown,
}

impl Into<HouseLocationType> for HouseType {
    fn into(self) -> HouseLocationType {
        match self {
            HouseType::Owned => HouseLocationType::Owned,
            HouseType::Rented => HouseLocationType::Rented,
            HouseType::Unknown => HouseLocationType::Unknown,
        }
    }
}

impl From<HouseLocationType> for HouseType {
    fn from(value: HouseLocationType) -> Self {
        match value {
            HouseLocationType::Owned => HouseType::Owned,
            HouseLocationType::Rented => HouseType::Rented,
            HouseLocationType::Unknown => HouseType::Unknown,
        }
    }
}

impl House {
    pub fn from_model(
        model: entity::house::Model,
        users: Option<Vec<entity::user::Model>>,
    ) -> Self {
        Self {
            id: model.id,
            name: model.name,
            address: model.address,
            country: model.country,
            code: model.code,
            city: model.city,
            zip_code: model.zip_code,
            description: model.description.unwrap_or_default(),
            built_year: model.built_year,
            location_type: model.location_type.into(),
            acquired_at: model.acquired_at,
            owner_name: model.owner_name,
            owner_email: model.owner_email,
            owner_phone: model.owner_phone,
            owner_contact_info: model.owner_contact_info,
            created_by: model.created_by,
            updated_by: model.updated_by,
            updated_at: model.updated_at,
            created_at: model.created_at,
            users: users.map(|users| {
                users
                    .into_iter()
                    .map(|user| User::from_model(user))
                    .collect()
            }),
        }
    }
}

//
// Get House Response
//
#[derive(Debug, Serialize)]
pub struct HouseResponse(pub House);

impl HouseResponse {
    pub fn new(house: entity::house::Model, users: Option<Vec<entity::user::Model>>) -> Self {
        HouseResponse(House::from_model(house, users))
    }
}

impl Responder for HouseResponse {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

//
// House list members Response
//
#[derive(Debug, Serialize)]
pub struct HouseUserResponse {
    pub users: Vec<User>,
}

//
// House Invite Response
//
pub struct HouseInviteResponse;

impl Responder for HouseInviteResponse {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().finish()
    }
}

//
// House Revoke Response
//
pub struct HouseRevokeResponse;

impl Responder for HouseRevokeResponse {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().finish()
    }
}

//
// House Revoke Response
//
pub struct HouseDeleteResponse;

impl Responder for HouseDeleteResponse {
    type Body = BoxBody;
    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().finish()
    }
}
