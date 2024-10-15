use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::Json;
use serde::de::DeserializeOwned;
use tracing::debug;
use validator::Validate;
use crate::exceptions::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dto<T: DeserializeOwned + Validate>(pub T);

impl<T: DeserializeOwned + Validate + 'static> FromRequest for Dto<T> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        debug!("Extracting DTO from request");
        let json_extract = Json::<T>::from_request(req, payload);
        Box::pin(async move {
            let value = json_extract.await.map_err(|x| {
                Error::Parse(x.to_string())
            })?.into_inner();
            value.validate()
                .map_err(|x| {
                    Error::Parse(x.to_string())
                })?;
            Ok(Dto (value))
        })
    }
}