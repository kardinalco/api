use actix_web::web::{delete, get, post, put, scope, Path, ServiceConfig};
use tracing::instrument;
use permission::resource::Resource;
use permission::role::RolePermission;
use crate::api::role::request::{CreateRoleBody, UpdateRole};
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::dto::Dto;
use crate::extractors::pagination::Pagination;
use crate::utils::response::{Response};
use crate::utils::route::Route;

use crate::api::role::response::Role as RoleResponse;
use crate::domain::role::RoleDomain;

pub struct RoleRoute;

pub type Role= Response<RoleResponse>;
pub type Roles = Response<Vec<RoleResponse>>;

impl RoleRoute {
    #[instrument]
    async fn list(session: AuthSession, pag: Pagination) -> Result<Roles, Error> {
        session.enforce(Resource::Role(RolePermission::Read)).await?;
        let roles =  RoleDomain::list(&session.db, pag).await?;
        Ok(Response::Ok(roles.into_iter().map(|x| RoleResponse::from_model(x)).collect()))
    }

    #[instrument]
    async fn create(session: AuthSession, body: Dto<CreateRoleBody>) -> Result<Role, Error> {
        session.enforce(Resource::Role(RolePermission::Create)).await?;
        let role = RoleDomain::create(session, body.into_inner()).await?;
        Ok(Response::Created(RoleResponse::from_model(role)))
    }

    #[instrument]
    async fn get(session: AuthSession, path: Path<String>) -> Result<Role, Error> {
        session.enforce(Resource::Role(RolePermission::Read)).await?;
        let role = RoleDomain::get(&session.db, &path.into_inner()).await?;
        Ok(Response::Ok(RoleResponse::from_model(role)))
    }

    #[instrument]
    async fn update(session: AuthSession, path: Path<String>, body: Dto<UpdateRole>) -> Result<Role, Error> {
        session.enforce(Resource::Role(RolePermission::Update)).await?;
        let role = RoleDomain::update(session, &path.into_inner(), body.into_inner()).await?;
        Ok(Response::Updated(RoleResponse::from_model((role, vec![]))))
    }

    #[instrument]
    async fn delete(session: AuthSession, path: Path<String>) -> Result<Role, Error> {
        session.enforce(Resource::Role(RolePermission::Delete)).await?;
        let role = RoleDomain::delete(&session, &path.into_inner()).await?;
        Ok(Response::Deleted(RoleResponse::from_model((role, vec![]))))
    }
    
}

impl Route for RoleRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/role")
                .route("", get().to(Self::list))
                .route("", post().to(Self::create))
                .route("/{id}", get().to(Self::get))
                .route("/{id}", put().to(Self::update))
                .route("/{id}", delete().to(Self::delete))
        );
    }
}