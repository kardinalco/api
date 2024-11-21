use actix_session::Session;
use actix_web::{Responder};
use actix_web::web::{post, scope};
use tracing::instrument;
use crate::api::auth::request::{AuthLoginRequest, AuthRegisterRequest};
use crate::domain::auth::AuthDomain;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;

pub struct AuthRoute;


impl AuthRoute {
    
    #[instrument(name = "auth::login", skip(s, body))]
    async fn login(body: Dto<AuthLoginRequest>, s: Session, db: DbReq) -> impl Responder {
        AuthDomain::login(body.0, db.0, s).await
    }

    #[instrument(name = "auth::register", skip(body))]
    async fn register(body: Dto<AuthRegisterRequest>, db: DbReq) -> impl Responder {
        AuthDomain::register(body.0, db.0).await
    }

    #[instrument(name = "auth::logout", skip(auth_session))]
    async fn logout(auth_session: AuthSession) -> impl Responder {
        AuthDomain::logout(auth_session).await
    }

    #[instrument]
    async fn forgot_password() -> impl Responder {
        ""
    }

    #[instrument]
    async fn reset_password() -> impl Responder {
        ""
    }

    #[instrument]
    async fn get_google_login_url() -> impl Responder {
        ""
    }

    #[instrument]
    async fn google_callback() -> impl Responder {
        ""
    }
    
}

impl Route for AuthRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/auth")
                .route("/login", post().to(AuthRoute::login))
                .route("/google/", post().to(AuthRoute::reset_password))
                .route("/google/callback", post().to(AuthRoute::reset_password))
                .route("/register", post().to(AuthRoute::register))
                .route("/logout", post().to(AuthRoute::logout))
                .route("/forgot-password", post().to(AuthRoute::forgot_password))
                .route("/reset-password", post().to(AuthRoute::reset_password))
        );
    }
}