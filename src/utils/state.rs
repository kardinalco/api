use sea_orm::{Database, DatabaseConnection};

use crate::exceptions::error::Error;
use crate::exceptions::db::DatabaseError;

use super::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub settings: Settings,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        let settings = Settings::new()?;
        Ok(Self {
            db: build_db(&settings.database.url).await?,
            settings,
        })
    }
}

pub async fn build_db(url: &String) -> Result<DatabaseConnection, DatabaseError> {
    Ok(Database::connect(url).await?)
}