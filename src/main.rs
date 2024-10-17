use actix_web::{web::Data, App, HttpServer};
use bb8::State;
use entity::user;
use utils::state::AppState;
use crate::api::auth::handler::AuthRoute;
use crate::utils::route::Route;

use crate::api::user::handler::UserRoute;

use sea_orm::{sqlx::{pool::PoolConnection, Postgres}, Database, DatabaseConnection, EntityTrait};

use actix_session::storage::RedisSessionStore;
use deadpool_redis::{Config, Runtime};

mod test_helpers;
mod utils;
mod api;
mod extractors;
mod exceptions;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let redis_cfg = Config::from_url("redis://127.0.0.1:6379");
    let redis_pool = redis_cfg.create_pool(Some(Runtime::Tokio1)).unwrap();
    
    let store = RedisSessionStore::new_pooled(redis_pool).await.unwrap();

    let db: DatabaseConnection = Database::connect("postgresql://nico:14142135@142.93.32.117:5432/postgres").await.unwrap();
    let a = user::Entity::find().all(&db).await.unwrap();

    let state: AppState = AppState::new().await.unwrap();

    match db.ping().await {
        Ok(a) => {
            println!("pong !");
        },
        Err(a) => {
            println!("database connection error !");
        }
    }

    tracing::info!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .configure(AuthRoute::route)
            .configure(UserRoute::route)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
