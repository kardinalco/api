use actix_session::Session;
use actix_web::{HttpResponse, Responder};
use actix_web::http::StatusCode;
use actix_web::web::{get, post, scope};
use tracing::instrument;
use crate::api::auth::request::{AuthLoginRequest, AuthLoginWithGoogleRequest, AuthRegisterRequest};
use crate::api::auth::response::{AuthLoginResponse, AuthRegisterResponse, GoogleGetUrlResponse, GoogleLoginResponse};
use crate::domain::auth::AuthDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::cache::Cache;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;

pub struct AuthRoute;


impl AuthRoute {
    
    #[instrument(skip(s, body, db))]
    async fn login(body: Dto<AuthLoginRequest>, s: Session, db: DbReq) -> Result<AuthLoginResponse, Error> {
        let user = AuthDomain::login(body.0, db.0, s).await?;
        Ok(AuthLoginResponse::new(user))
    }

    #[instrument(skip(body))]
    async fn register(body: Dto<AuthRegisterRequest>, db: DbReq) -> Result<AuthRegisterResponse, Error> {
        AuthDomain::register(body.0, db.0).await?;
        Ok(AuthRegisterResponse { message: "User registered successfully" })
    }

    #[instrument(skip(auth_session))]
    async fn logout(auth_session: AuthSession) -> Result<HttpResponse, Error> {
        AuthDomain::logout(auth_session).await?;
        Ok(HttpResponse::new(StatusCode::OK))
    }

    #[instrument]
    async fn forgot_password() -> impl Responder {
        ""
    }

    #[instrument]
    async fn reset_password() -> impl Responder {
        ""
    }

    #[instrument(skip(db, cache))]
    async fn get_google_login_url(db: DbReq, cache: Cache) -> Result<GoogleGetUrlResponse, Error> {
        let result = AuthDomain::build_google_auth_url(&db.0, &cache.0).await?;
        Ok(GoogleGetUrlResponse::new("Google login url".to_string(), result))
    }

    #[instrument(skip(db, cache, s))]
    async fn google_login(db: DbReq, cache: Cache, s: Session, body: Dto<AuthLoginWithGoogleRequest>) -> Result<GoogleLoginResponse, Error> {
        let user = AuthDomain::login_with_google(&db.0, &cache.0, &s, &body.into_inner().code).await?;
        Ok(GoogleLoginResponse::new(user))
    }
    
}

impl Route for AuthRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/auth")
                .route("/login", post().to(AuthRoute::login))
                .route("/google", get().to(AuthRoute::get_google_login_url))
                .route("/google", post().to(AuthRoute::google_login))
                .route("/register", post().to(AuthRoute::register))
                .route("/logout", post().to(AuthRoute::logout))
                .route("/forgot-password", post().to(AuthRoute::forgot_password))
                .route("/reset-password", post().to(AuthRoute::reset_password))
        );
    }
}