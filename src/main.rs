#![allow(dead_code)]

use actix_cors::Cors;
use crate::api::auth::handler::AuthRoute;
use crate::utils::route::Route;
use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_web::{web::Data, App, HttpServer};
use utils::state::AppState;

use crate::api::user::handler::UserRoute;

mod api;
mod domain;
mod exceptions;
mod extractors;
mod middleware;
mod services;
mod test_helpers;
mod utils;
mod entity;

use crate::api::credentials::handler::CredentialsRoute;
use crate::api::expense::handler::ExpenseRoute;
use crate::api::house::handler::HouseRoute;
use crate::api::permission::handler::PermissionRoute;
use actix_web::cookie::time::Duration;
use actix_web::cookie::Key;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let state: AppState = AppState::new().await.map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to start server")
    })?;
    
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default()) // Ajoute le middleware de tracing
            .wrap(Cors::default())
            .wrap(SessionMiddleware::builder(
                    state.session_store.clone(),
                    Key::from(state.settings.keys.session.as_bytes())).session_lifecycle(PersistentSession::default().session_ttl(Duration::seconds(60 * 60 * 24 * 2)))
                .build())
            .app_data(Data::new(state.clone()))
            .configure(AuthRoute::route)
            .configure(UserRoute::route)
            .configure(HouseRoute::route)
            .configure(CredentialsRoute::route)
            .configure(ExpenseRoute::route)
            .configure(PermissionRoute::route)
            .wrap(NormalizePath::new(TrailingSlash::Trim))
    })
    .bind(format!("0.0.0.0:{}", 3001))?
    .run()
    .await
}
