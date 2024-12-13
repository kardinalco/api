use actix_multipart::form::tempfile::TempFile;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use s3::{Bucket, Region};
use s3::creds::Credentials;
use sea_orm::DatabaseConnection;
use tokio::fs::File;
use tracing::instrument;
use uuid::{Uuid};
use crate::exceptions::error::Error;
use crate::exceptions::settings::SettingsError;
use crate::services::cache::CachedSettings;
use crate::utils::settings::Settings;

pub struct StorageService;

impl StorageService {
    
    #[instrument(skip(db, cache))]
    async fn get_bucket(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, bucket_name: &str) -> Result<Box<Bucket>, Error> {
        let settings = Settings::<settings::bucket::Bucket>::new(cache, db).await?.into_inner();
        let region = Region::Custom {
            region: settings.region.name,
            endpoint: settings.region.endpoint,
        };
        let credentials= settings.info.into_iter().find(|info| info.name == bucket_name).map(|info| {
            Credentials::new(Some(&(*info.user)), Some(&(*info.key)), None, None, None)
        }).ok_or(Error::Settings(SettingsError::WrongBucketName(String::from(bucket_name))))?;
        Ok(Bucket::new(bucket_name, region, credentials?)?.with_path_style())
    }

    #[instrument(skip(db, cache))]
    pub async fn upload_user_profile_picture(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, file: TempFile) -> Result<String, Error> {
        let bucket = Self::get_bucket(db, cache, "user-profiles").await?;
        let mut async_file = File::open(file.file.path()).await?;
        let code = format!("{}.png", Uuid::new_v4().to_string());
        match file.content_type {
            Some(content_type) => bucket.put_object_stream_with_content_type(&mut async_file, &code, content_type).await?,
            None => bucket.put_object_stream(&mut async_file, Uuid::new_v4().to_string()).await?,
        };
        Ok(code)
    }

    #[instrument(skip(db, cache))]
    pub async fn delete_user_profile_picture(db: &DatabaseConnection, cache: &Pool<RedisConnectionManager>, code: &str) -> Result<(), Error> {
        let bucket = Self::get_bucket(db, cache, "user-profiles").await?;
        bucket.delete_object(code).await?;
        Ok(())
    }
}