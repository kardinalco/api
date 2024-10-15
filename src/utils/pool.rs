use actix_session::storage::RedisSessionStore;
use diesel::r2d2::ConnectionManager;
use r2d2_redis::RedisConnectionManager;
use crate::exceptions::cache::CacheError;
use crate::exceptions::db::DatabaseError;
use crate::exceptions::error::Error;
use std::thread;
use deadpool_redis::Config;
use deadpool_redis::Runtime::Tokio1;
use diesel::prelude::*;
use r2d2::Pool;

pub fn build_redis_pool(url: &str) -> Result<(), CacheError> {
    let pool = Config::from_url(url).create_pool(Some(Tokio1));
    Ok(())
}

pub fn build_db_pool(url: &str) -> Result<Pool<ConnectionManager<PgConnection>>, DatabaseError> {
    let manager = ConnectionManager::<PgConnection>::new(url);
    Ok(Pool::builder().build(manager)?)
}