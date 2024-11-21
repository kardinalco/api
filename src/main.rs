use actix_session::config::PersistentSession;
use actix_session::SessionMiddleware;
use actix_web::{web::Data, App, HttpServer};
use utils::state::AppState;
use crate::api::auth::handler::AuthRoute;
use crate::utils::route::Route;

use crate::api::user::handler::UserRoute;

use actix_session::storage::RedisSessionStore;

mod test_helpers;
mod utils;
mod api;
mod extractors;
mod exceptions;
mod services;
mod domain;

use actix_web::cookie::Key;
use actix_web::cookie::time::Duration;
use bb8_redis::RedisConnectionManager;
use crate::api::house::handler::HouseRoute;
use crate::domain::role::RoleDomain;
use crate::exceptions::error::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let manager = RedisConnectionManager::new("/***/").unwrap();
    let _pool = bb8::Pool::builder().build(manager).await.unwrap();
    let redis_store = RedisSessionStore::new("/***/").await.map_err(|e| Error::InternalServer(e.to_string())).unwrap();
    let state: AppState = AppState::new().await.unwrap();

/*    let adapter = RbacAdapter::new(Arc::new(state.db.clone()), Arc::new(pool.clone()));
    let e = Enforcer::new("rbac_model.conf", adapter).await.unwrap();
    
    println!("{:?}", e.get_all_policy());
    
    let result = e.enforce(("Admin", "user", "read")).unwrap();
    println!("{:?}", result);*/

    RoleDomain::get_roles_and_permissions(&state.db).await.unwrap();
    
    let settings = state.settings.clone();
    println!("Starting server");
    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    redis_store.clone(),
                    Key::from(settings.keys.session.as_bytes()),
                ).session_lifecycle(PersistentSession::default().session_ttl(Duration::seconds(60 * 60 * 24 * 2))).build()
            )
            .app_data(Data::new(state.clone()))
            . configure(AuthRoute::route)
            .configure(UserRoute::route)
            .configure(HouseRoute::route)
    })
        .bind(format!("0.0.0.0:{}", settings.api.port))?
        .run()
        .await
}
