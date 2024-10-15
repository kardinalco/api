use actix_web::{Responder};
use actix_web::web::{post, scope};
use tracing::instrument;
use crate::api::auth::request::{AuthLoginRequest, AuthRegisterRequest};
use crate::exceptions::auth::AuthenticateError;
use crate::exceptions::error::Error;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;

pub struct AuthRoute;

impl AuthRoute {
    
    #[instrument(name = "auth::login", skip(body))]
    async fn login(body: Dto<AuthLoginRequest>) -> impl Responder {
        Error::Auth(AuthenticateError::WrongCredentials)
    }

    #[instrument(name = "auth::register", skip(body))]
    async fn register(body: Dto<AuthRegisterRequest>) -> impl Responder {
        ""
    }

    #[instrument]
    async fn logout() -> impl Responder {
        ""
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