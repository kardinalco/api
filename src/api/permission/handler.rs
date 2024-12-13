use crate::utils::route::Route;
use actix_web::web::{scope, Path, ServiceConfig};
use tracing::instrument;
use permission::permission::Permissions;
use permission::resource::Resource;
use crate::api::permission::response::Permission;
use crate::domain::permission::PermissionDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::pagination::Pagination;
use crate::utils::response::Response;

pub struct PermissionRoute;

impl PermissionRoute {

    #[instrument(name = "handler::get")]
    pub async fn get(session: AuthSession, path: Path<String>) -> Result<Response<Permission>, Error> {
        session.enforce(Resource::Permission(Permissions::Read)).await?;
        let permission = PermissionDomain::find_permission_by_id(&session.db, &path).await?;
        Ok(Response::Ok(permission.into()))
    }

    #[instrument(name = "handler::list")]
    pub async fn list(session: AuthSession, pag: Pagination) -> Result<Response<Vec<Permission>>, Error> {
        session.enforce(Resource::Permission(Permissions::List)).await?;
        let permissions = PermissionDomain::list(&session.db, pag).await?;
        Ok(Response::Ok(permissions.into_iter().map(|p| p.into()).collect()))
    }
}

impl Route for PermissionRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/permission")
                .route("/", actix_web::web::get().to(Self::list))
                .route("/{id}", actix_web::web::get().to(Self::get))
        );
    }
}
