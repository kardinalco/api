use crate::exceptions::error::Error;
use actix_web::dev::Payload;
use actix_web::web::Json;
use actix_web::{FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use std::future::Future;
use std::pin::Pin;
use tracing::instrument;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dto<T: DeserializeOwned + Validate>(T);

impl<T: DeserializeOwned + Validate> Dto<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: DeserializeOwned + Validate + 'static> FromRequest for Dto<T> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    #[instrument(name = "dto::from_request", skip(req, payload))]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_extract = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            let value = json_extract
                .await
                .map_err(|x| Error::Parse(x.to_string()))?
                .into_inner();
            value.validate().map_err(|x| Error::Parse(x.to_string()))?;
            Ok(Dto(value))
        })
    }
}