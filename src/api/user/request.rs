use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::exceptions::error::Error;
use crate::exceptions::request::RequestError;

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

#[derive(Debug, Validate, MultipartForm)]
pub struct UploadProfilePictureRequest {
    #[multipart(limit = "5M")]
    pub file: TempFile,
    pub name: Text<String>,
}

impl UploadProfilePictureRequest {
    pub fn verify_content_type(&self) -> Result<(), Error> {
        match self.file.content_type {
            Some(ref content_type) => {
                match content_type.type_() {
                    mime::IMAGE => match content_type.subtype() {
                        mime::JPEG | mime::PNG => Ok(()),
                        _ => Err(Error::Request(RequestError::InvalidMimeType)),
                    },
                    _ => Err(Error::Request(RequestError::InvalidMimeType)),
                }
            },
            _ => Err(Error::Request(RequestError::InvalidMimeType)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserDeleteRequest {
    pub users: Vec<String>
}