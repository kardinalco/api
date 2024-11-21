use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::{Serialize};
use crate::utils::response::Response;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub firstname: String,
    pub lastname: String,
    pub created_at: chrono::NaiveDateTime,
    pub city: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub zip_code: Option<String>,
    pub phone_number: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    pub picture: Option<String>,
}

impl User {
    pub fn from_model(user: entity::user::Model) -> Self {
        User {
            id: user.id.clone(),
            email: user.email.clone(),
            firstname: user.first_name.clone(),
            lastname: user.last_name.clone(),
            created_at: user.created_at,
            city: user.city.clone(),
            country: user.country.clone(),
            address: user.address.clone(),
            zip_code: user.zip_code.clone(),
            phone_number: user.phone_number.clone(),
            birthday: user.birthday,
            picture: user.picture.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserListResponse(pub Vec<User>);

impl UserListResponse {
    pub fn new(users: Vec<entity::user::Model>) -> Self {
        UserListResponse(users.into_iter().map(User::from_model).collect())
    }
}

impl Responder for UserListResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}