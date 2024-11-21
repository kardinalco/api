use std::future::Future;
use std::pin::Pin;
use entity::prelude::User;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use sea_orm::EntityTrait;
use crate::exceptions::auth::AuthenticateError::NeedSession;
use crate::exceptions::error::Error;
use crate::extractors::db::DbReq;

pub struct AuthSession {
    pub user: entity::user::Model,
    pub session: Session,
}

impl FromRequest for AuthSession {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone(); 
        Box::pin(async move {
            let session = Session::extract(&request).await?;
            let db = DbReq::extract(&request).await?;
            let user_id = session.get::<String>("user_id")?.ok_or(Error::Auth(NeedSession))?;
            let user = User::find_by_id(user_id).one(&db.0).await
                .map_err(|_| Error::Auth(NeedSession))?
                .ok_or(Error::Auth(NeedSession))?;
            Ok(AuthSession { user, session })
        })
    }
}