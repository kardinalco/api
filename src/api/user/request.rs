use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserUpdateRequest {
    #[validate(length(min = 2, max = 64))]
    pub firstname: Option<String>,
    #[validate(length(min = 2, max = 64))]
    pub lastname: Option<String>,
    #[validate(length(min = 6, max = 20))]
    pub phone_number: Option<String>,
    pub birthday: Option<chrono::NaiveDate>,
    #[validate(length(min = 2, max = 64))]
    pub country: Option<String>,
    #[validate(length(min = 2, max = 100))]
    pub city: Option<String>,
    #[validate(length(min = 2, max = 100))]
    pub address: Option<String>,
    #[validate(length(min = 2, max = 6))]
    pub zip_code: Option<String>,
}
