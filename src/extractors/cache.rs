use crate::exceptions::error::Error;
use crate::utils::state::AppState;
use actix_web::dev::Payload;
use actix_web::web::Data;
use actix_web::{FromRequest, HttpRequest};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use std::future::Future;
use std::pin::Pin;

pub struct Cache(Pool<RedisConnectionManager>);

impl FromRequest for Cache {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let data = request
                .app_data::<Data<AppState>>()
                .ok_or(Error::InternalServer(String::from("value")))?;
            Ok(Cache(data.cache.clone()))
        })
    }
}

impl Cache {
    pub(crate) fn into_inner(self) -> Pool<RedisConnectionManager> {
        self.0
    }
}
