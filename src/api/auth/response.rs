use crate::api::user::response::User;
use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthLoginResponse {
    pub message: &'static str,
    pub user: User,
}

impl AuthLoginResponse {
    pub fn new(user: entity::user::Model) -> Self {
        AuthLoginResponse {
            message: "Successfully logged in",
            user: User::from_model(user),
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

#[derive(Debug, Serialize)]
pub struct GoogleGetUrlResponse {
    pub message: String,
    pub url: String,
}

impl GoogleGetUrlResponse {
    pub fn new(message: String, url: String) -> Self {
        GoogleGetUrlResponse { message, url }
    }
}

impl Responder for GoogleGetUrlResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct GoogleLoginResponse {
    pub message: &'static str,
    pub user: User,
}

impl GoogleLoginResponse {
    pub fn new(user: entity::user::Model) -> Self {
        Self {
            user: User::from_model(user),
            message: "Successfully logged in with Google",
        }
    }
}

impl Responder for GoogleLoginResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthVerifyResponse {
    pub message: &'static str,
    pub user: User,
}

impl AuthVerifyResponse {
    pub fn new(user: entity::user::Model) -> Self {
        AuthVerifyResponse {
            message: "Verification successful",
            user: User::from_model(user),
        }
    }
}

impl Responder for AuthVerifyResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}