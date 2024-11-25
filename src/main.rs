use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_web::{web::Data, App, HttpServer};
use utils::state::AppState;
use crate::api::auth::handler::AuthRoute;
use crate::utils::route::Route;

use crate::api::user::handler::UserRoute;

mod test_helpers;
mod utils;
mod api;
mod extractors;
mod exceptions;
mod services;
mod domain;

use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use crate::api::house::handler::HouseRoute;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state: AppState = AppState::new().await.map_err(|e| {
        println!("Error: {:?}", e);
        std::io::Error::new(std::io::ErrorKind::Other, "Failed to start server")
    })?;
    let settings = state.settings.clone();
    println!("Starting server ...");
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    state.session_store.clone(),
                    Key::from(settings.keys.session.as_bytes()),
                ).session_lifecycle(PersistentSession::default().session_ttl(Duration::seconds(60 * 60 * 24 * 2))).build()
            )
            .app_data(Data::new(state.clone()))
            .configure(AuthRoute::route)
            .configure(UserRoute::route)
            .configure(HouseRoute::route)
    })
        .bind(format!("0.0.0.0:{}", settings.api.port))?
        .run()
        .await
}
