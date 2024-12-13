use crate::api::auth::request::{AuthForgotPasswordRequest, AuthLoginRequest, AuthLoginWithGoogleRequest, AuthRegisterRequest, AuthResetPasswordRequest, AuthVerifyRequest};
use crate::api::auth::response::{AuthLoginResponse, AuthRegisterResponse, AuthVerifyResponse, GoogleGetUrlResponse, GoogleLoginResponse};
use crate::domain::auth::AuthDomain;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::extractors::cache::Cache;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;
use crate::utils::route::Route;
use actix_session::Session;
use actix_web::http::StatusCode;
use actix_web::web::{get, post, scope};
use actix_web::{HttpResponse};
use tracing::instrument;

pub struct AuthRoute;

impl AuthRoute {
    #[instrument(name = "handler::login", skip(s, body, db))]
    async fn login(body: Dto<AuthLoginRequest>, s: Session, db: DbReq) -> Result<AuthLoginResponse, Error> {
        let user = AuthDomain::login(body.into_inner(), db.into_inner(), s).await?;
        Ok(AuthLoginResponse::new(user))
    }

    #[instrument(name = "handler::register", skip(body, db, cache))]
    async fn register(body: Dto<AuthRegisterRequest>, db: DbReq, cache: Cache) -> Result<AuthRegisterResponse, Error> {
        AuthDomain::register(body.into_inner(), db.into_inner(), cache.into_inner()).await?;
        Ok(AuthRegisterResponse { message: "User registered successfully" })
    }

    #[instrument(name = "handler::verify", skip(body, db, cache))]
    async fn verify(db: DbReq, cache: Cache, body: Dto<AuthVerifyRequest>) -> Result<AuthVerifyResponse, Error> {
        let user = AuthDomain::verify_user(&db.into_inner(), &cache.into_inner(), &body.into_inner().code).await?;
        Ok(AuthVerifyResponse::new(user))
    }

    #[instrument(name = "handler::logout")]
    async fn logout(auth_session: AuthSession) -> Result<HttpResponse, Error> {
        AuthDomain::logout(auth_session).await?;
        Ok(HttpResponse::new(StatusCode::OK))
    }

    #[instrument(name = "handler::forgot_password", skip(body, db, cache))]
    async fn forgot_password(db: DbReq, cache: Cache, body: Dto<AuthForgotPasswordRequest>) -> Result<HttpResponse, Error> {
        AuthDomain::forgot_password(&db.into_inner(), &cache.into_inner(), &body.into_inner().email).await?;
        Ok(HttpResponse::new(StatusCode::OK))
    }

    #[instrument(name = "handler::reset_password", skip(body, db, cache))]
    async fn reset_password(db: DbReq, cache: Cache, body: Dto<AuthResetPasswordRequest>) -> Result<HttpResponse, Error> {
        AuthDomain::reset_password(&db.into_inner(), &cache.into_inner(), &body.into_inner()).await?;
        Ok(HttpResponse::new(StatusCode::OK))
    }

    #[instrument(name = "handler::get_google_login_url", skip(db, cache))]
    async fn get_google_login_url(db: DbReq, cache: Cache) -> Result<GoogleGetUrlResponse, Error> {
        let result = AuthDomain::build_google_auth_url(&db.into_inner(), &cache.into_inner()).await?;
        Ok(GoogleGetUrlResponse::new(
            "Google login url".to_string(),
            result,
        ))
    }

    #[instrument(name = "handler::google_login", skip(db, cache, s))]
    async fn google_login(db: DbReq, cache: Cache, s: Session, body: Dto<AuthLoginWithGoogleRequest>) -> Result<GoogleLoginResponse, Error> {
        let user = AuthDomain::login_with_google(&db.into_inner(), &cache.into_inner(), &s, &body.into_inner().code).await?;
        Ok(GoogleLoginResponse::new(user))
    }
}

impl Route for AuthRoute {
    fn route(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            scope("/auth")
                .route("/register", post().to(AuthRoute::register))
                .route("/login", post().to(AuthRoute::login))
                .route("/verify", post().to(AuthRoute::verify))
                .route("/google", get().to(AuthRoute::get_google_login_url))
                .route("/google", post().to(AuthRoute::google_login))
                .route("/logout", post().to(AuthRoute::logout))
                .route("/forgot-password", post().to(AuthRoute::forgot_password))
                .route("/reset-password", post().to(AuthRoute::reset_password)),
        );
    }
}
