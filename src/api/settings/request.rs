use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

#[derive(Debug, Serialize, Deserialize)]
pub enum SettingsResetCache {
    All,
    Bucket,
    Global,
    Google,
    Mail,
}

impl Validate for SettingsResetCache {
    fn validate(&self) -> Result<(), ValidationErrors> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsResetCacheBody {
    pub cache: SettingsResetCache,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpdateSettings {
    Bucket(BucketSettings),
    Global(GlobalSettings),
    Google(GoogleSettings),
    Mail(MailSettings),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketSettings {
    pub bucket: String,
    pub settings: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalSettings {
    pub settings: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleSettings {
    pub settings: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailSettings {
    pub settings: String,
}