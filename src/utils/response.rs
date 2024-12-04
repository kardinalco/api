use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

pub enum Response<T: Serialize> {
    Ok(T),
    Created(T),
    Updated(T),
    Deleted(T),
    NoContent,
}

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Response::Ok(data) => HttpResponse::Ok().json(data),
            Response::Created(data) => HttpResponse::Created().json(data),
            Response::Updated(data) => HttpResponse::Ok().json(data),
            Response::Deleted(data) => HttpResponse::Ok().json(data),
            Response::NoContent => HttpResponse::NoContent().finish(),
        }
    }
}
