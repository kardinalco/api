use crate::exceptions::error::Error;
use crate::utils::state::AppState;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use sea_orm::DatabaseConnection;
use std::future::Future;
use std::pin::Pin;
use tracing::instrument;

#[derive(Clone, Debug)]
pub struct DbReq(DatabaseConnection);

impl DbReq {
    pub fn into_inner(self) -> DatabaseConnection {
        self.0
    }
}

impl FromRequest for DbReq {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    #[instrument(level = "info", name = "db::from_request", skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let data = request
                .app_data::<Data<AppState>>()
                .ok_or(Error::InternalServer(String::from("value")))?;
            Ok(DbReq(data.db.clone()))
        })
    }
}
