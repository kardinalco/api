use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use entity::prelude::User;
use actix_session::Session;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use futures_util::future;
use permission::resource::Resource;
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::instrument;
use crate::exceptions::auth::AuthenticateError::NeedSession;
use crate::exceptions::error::Error;
use crate::extractors::cache::Cache;
use crate::extractors::db::DbReq;
use crate::services::permission::Permission;

pub struct AuthSession {
    pub user: entity::user::Model,
    pub session: Session,
    pub permission: Permission,
    pub db: DatabaseConnection,
}

impl AuthSession {
    
    pub async fn enforce_and(&self, resource: Vec<Resource>) -> Result<(), Error> {
        future::try_join_all(resource.iter().map(|r| self.enforce(*r))).await?;
        Ok(())
    }
    
    pub async fn enforce_or(&self, resource: Vec<Resource>) -> Result<(), Error> {
        let a = future::join_all(resource.iter().map(|r| self.enforce(*r))).await;
        if a.iter().any(|r| r.is_ok()) {
            Ok(())
        } else {
            Err(Error::Auth(NeedSession))
        }
    }
    
    pub async fn enforce(&self, resource: Resource) -> Result<(), Error> {
        self.permission.enforce((&self.user, &resource.get_resource(), &resource.get_action())).await
    }
    
    pub async fn enforce_str(&self, (obj, action): (&str, &str)) -> Result<(), Error> {
        self.permission.enforce((&self.user, obj, action)).await
    }
}

impl FromRequest for AuthSession {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    #[instrument(skip(req))]
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
            Ok(AuthSession { user, session, permission: Permission::new(Arc::new(db.clone()), Arc::new(cache.into_inner())), db})
        })
    }
}
