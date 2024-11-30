use std::future::{ready, Ready};

use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::error::Error;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use futures_util::future::LocalBoxFuture;

pub struct Provider;

impl<S, R> Transform<S, ServiceRequest> for Provider
where
    S: Service<ServiceRequest, Response = ServiceResponse<R>, Error = Error>,
    S::Future: 'static,
    R: 'static,
{
    type Response = ServiceResponse<R>;
    type Error = Error;
    type Transform = ProviderMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ProviderMiddleware { service }))
    }
}

pub struct ProviderMiddleware<S> {
    service: S,
}

impl<S, R> Service<ServiceRequest> for ProviderMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<R>, Error = Error>,
    S::Future: 'static,
    R: 'static,
{
    type Response = ServiceResponse<R>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);
    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Do something with the request here
        if let Some(provider) = req.headers().get("App-Provider") {
            let _v = Box::pin(async move {
                println!("{:?}", provider.to_str());
            });
        } else {
            //if not return an error
            return Box::pin(async { Err(Error::from(AuthenticateError::WrongCredentials)) });
        }
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
