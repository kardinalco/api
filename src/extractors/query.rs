use std::future::Future;
use std::pin::Pin;
use actix_web::FromRequest;
use crate::exceptions::error::Error;

pub struct Query {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

impl FromRequest for Query {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(_req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
       todo!()
    }
}