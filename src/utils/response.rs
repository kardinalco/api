use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

pub struct Response<T: Serialize>(pub T);

impl<T: Serialize> Responder for Response<T> {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self.0)
    }
}
