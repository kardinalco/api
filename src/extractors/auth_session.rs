use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use entity::prelude::User;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures_util::future;
use permission::resource::Resource;
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::instrument;
use crate::exceptions::auth::AuthenticateError::NeedSession;
use crate::exceptions::error::Error;
use crate::extractors::cache::Cache;
use crate::extractors::db::DbReq;
use crate::services::permission::{Permission};

pub struct AuthSession {
    pub user: entity::user::Model,
    pub session: Session,
    pub permission: Permission,
    pub db: DatabaseConnection,
    pub cache: Pool<RedisConnectionManager>,
}

impl Debug for AuthSession {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthSession")
            .field("user", &self.user)
            .finish()
    }
}

impl AuthSession {
    
    #[instrument(level = "info", name = "auth_session::enforce_and", skip(self))]
    pub async fn enforce_and(&self, resource: Vec<Resource>) -> Result<(), Error> {
        future::try_join_all(resource.iter().map(|r| self.enforce(*r))).await?;
        Ok(())
    }
    
    #[instrument(level = "info", name = "auth_session::enforce_or", skip(self))]
    pub async fn enforce_or(&self, resource: Vec<Resource>) -> Result<(), Error> {
        let permissions = resource.iter().map(|x| (x.to_string(), x.get_action())).collect::<Vec<(String, String)>>();
        self.permission.enforce_or(&self.user, permissions).await
    }
    
    #[instrument(level = "info", name = "auth_session::enforce", skip(self))]
    pub async fn enforce(&self, resource: Resource) -> Result<(), Error> {
        self.permission.enforce((&self.user, &resource.get_resource(), &resource.get_action())).await
    }
    
    #[instrument(level = "info", name = "auth_session::enforce", skip(self))]
    pub async fn enforce_str(&self, (obj, action): (&str, &str)) -> Result<(), Error> {
        self.permission.enforce((&self.user, obj, action)).await
    }
}

impl FromRequest for AuthSession {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    #[instrument(level = "info", name = "auth_session::from_request", skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone(); 
        Box::pin(async move {
            let session = Session::extract(&request).await?;
            let cache = Cache::extract(&request).await?;
            let db = DbReq::extract(&request).await?;
            let user_id = session.get::<String>("user_id")?.ok_or(Error::Auth(NeedSession))?;
            let db = db.into_inner();
            let user = User::find_by_id(user_id).one(&db).await
                .map_err(|_| Error::Auth(NeedSession))?
                .ok_or(Error::Auth(NeedSession))?;
            Ok(AuthSession { user, session, permission: Permission::new(Arc::new(db.clone()), Arc::new(cache.clone().into_inner())), db, cache: cache.into_inner()})
        })
    }
}
