use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::exceptions::error::Error;
use crate::utils::state::AppState;
use sea_orm::DatabaseConnection;
use actix_web::web::Data;

#[derive(Debug, Clone)]
pub struct DbReq(pub DatabaseConnection);

impl FromRequest for DbReq {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let data = request.app_data::<Data<AppState>>();
            Ok(DbReq(todo!()))
        })
    }
}