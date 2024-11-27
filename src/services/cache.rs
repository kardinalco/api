use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, FromRedisValue, JsonAsyncCommands};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::json;
use crate::exceptions::error::Error;

pub trait CachedSettings: Sized + DeserializeOwned + Serialize + FromRedisValue + Send + Sync {

    fn get_key() -> String;

    fn get_ttl() -> i64 {
        3600
    }
    
    async fn get(redis: &Pool<RedisConnectionManager>, db: &DatabaseConnection) -> Result<Self, Error> {
        match Self::get_from_cache(redis).await {
            Ok(settings) => { 
                Ok(settings) 
            },
            Err(_) => {
                let result = Self::get_from_db(db).await?;
                result.save_to_cache(redis).await?;
                Ok(result)
            },
        }
    }

    async fn get_from_cache(redis: &Pool<RedisConnectionManager>) -> Result<Self, Error> {
        let mut a = redis.get().await?;
        Ok(a.json_get(Self::get_key(), ".").await?)
    }

    async fn get_from_db(db: &DatabaseConnection) -> Result<Self, Error> {
        let result = entity::settings::Entity::find()
            .filter(entity::settings::Column::Name.eq(Self::get_key()))
            .one(db).await?.map_or(Err(Error::InternalServer(format!("Cannot found settings '{}' from db !", Self::get_key()))), Ok)?;
        Ok(serde_json::from_value(result.value)
            .map_err(|_| Error::InternalServer(format!("Cannot deserialize value of '{}' settings", Self::get_key())))?)
    }
    
    async fn update_to_db(&self, db: &DatabaseConnection) -> Result<(), Error> {
        let settings = entity::settings::Entity::find()
            .filter(entity::settings::Column::Name.eq(Self::get_key()))
            .one(db).await?.ok_or(Error::InternalServer(format!("Cannot found settings '{}' from db !", Self::get_key())))?;
        let mut model = settings.into_active_model();
        model.set(entity::settings::Column::Value, json!(self).into());
        model.set(entity::settings::Column::UpdatedAt, chrono::Local::now().into());
        model.update(db).await?;
        Ok(())
    }

    #[allow(dependency_on_unit_never_type_fallback)]
    async fn save_to_cache(&self, redis: &Pool<RedisConnectionManager>) -> Result<(), Error> {
        let mut conn = redis.get().await?;
        conn.json_set(Self::get_key(), ".", self).await?;
        conn.expire(Self::get_key(), Self::get_ttl()).await?;
        Ok(())
    }
}