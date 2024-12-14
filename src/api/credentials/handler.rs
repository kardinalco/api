use crate::exceptions::error::Error;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, ServiceConfig, Path};
use tracing::instrument;
use permission::credential::CredentialPermission;
use permission::resource::Resource;
use crate::api::credentials::request::{CredentialCreate, CredentialUpdate};
use crate::api::credentials::response::{CreateCredentialResponse, Credential, DeleteCredentialResponse, GetCredentialResponse, UpdateCredentialResponse};
use crate::domain::credential::CredentialDomain;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::dto::Dto;

pub struct CredentialsRoute;

impl CredentialsRoute {

    #[instrument]
    async fn list_credentials(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }
    
    #[instrument]
    async fn create_credentials(session: AuthSession, body: Dto<CredentialCreate>) -> Result<CreateCredentialResponse, Error> {
        session.enforce(Resource::Credential(CredentialPermission::Create)).await?;
        let result = CredentialDomain::create_credentials(&session.db, &session.user.id, body.into_inner()).await?;
        Ok(CreateCredentialResponse(Credential::from_model(result)))
    }

    #[instrument]
    async fn get_credentials(session: AuthSession, id: Path<String>) -> Result<GetCredentialResponse, Error> {
        session.enforce_or(vec![Resource::Credential(CredentialPermission::ReadSelf), Resource::Credential(CredentialPermission::Read)]).await?;
        let result = CredentialDomain::get_credentials(&session.db, &id).await?;
        Ok(GetCredentialResponse(Credential::from_model(result)))
    }

    #[instrument]
    async fn update_credentials(session: AuthSession, id: Path<String>, body: Dto<CredentialUpdate>) -> Result<UpdateCredentialResponse, Error> {
        session.enforce_or(vec![Resource::Credential(CredentialPermission::UpdateSelf), Resource::Credential(CredentialPermission::Update)]).await?;
        let result = CredentialDomain::update_credentials(&session.db, &session.user.id, &id.into_inner(), body.into_inner()).await?;
        Ok(UpdateCredentialResponse(Credential::from_model(result)))
    }

    #[instrument]
    async fn delete_credentials(session: AuthSession, id: Path<String>) -> Result<DeleteCredentialResponse, Error> {
        session.enforce_or(vec![Resource::Credential(CredentialPermission::DeleteSelf), Resource::Credential(CredentialPermission::Delete)]).await?;
        let result = CredentialDomain::delete_credentials(&session.db, &session.user.id, &id.into_inner()).await?;
        Ok(DeleteCredentialResponse(Credential::from_model(result)))
    }
}

impl Route for CredentialsRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/credentials")
                .route("", get().to(Self::list_credentials))
                .route("", post().to(Self::create_credentials))
                .route("{id}", get().to(Self::get_credentials))
                .route("{id}", put().to(Self::update_credentials))
                .route("{id}", delete().to(Self::delete_credentials))
        );
            
    }
}
