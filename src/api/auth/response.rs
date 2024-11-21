use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::Serialize;
use crate::api::user::response::User;

#[derive(Debug, Serialize)]
pub struct AuthLoginResponse {
    pub message: &'static str,
    pub user: User
}

impl AuthLoginResponse {
    pub fn new(user: entity::user::Model) -> Self {
        AuthLoginResponse {
            message: "Successfully logged in",
            user: User::from_model(user)
        }
    }
}

impl Responder for AuthLoginResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthRegisterResponse {
    pub message: &'static str,
}

impl Responder for AuthRegisterResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}