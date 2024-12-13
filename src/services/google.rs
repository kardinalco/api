use crate::exceptions::request::RequestError;
use serde::Deserialize;
use settings::google::Google;
use tracing::instrument;

pub struct GoogleService;

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub refresh_token: Option<String>,
    pub scope: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GoogleUserInfoResponse {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub given_name: String,
    pub family_name: Option<String>,
    pub name: String,
}

impl GoogleService {
    #[instrument]
    pub async fn login(google: &Google, code: &str) -> Result<GoogleTokenResponse, RequestError> {
        let params = [
            ("code", code),
            ("client_id", google.get_client_id()),
            ("client_secret", google.get_client_secret()),
            ("redirect_uri", google.get_redirect_uri()),
            ("grant_type", google.get_grant_type()),
        ];
        let client = reqwest::Client::new();
        let result = client
            .post(google.get_url().get_token_url())
            .form(&params)
            .send()
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(result.json::<GoogleTokenResponse>().await?),
            reqwest::StatusCode::BAD_REQUEST => Err(RequestError::GoogleInvalidState),
            _ => Err(RequestError::Internal(result.text().await?)),
        }
    }

    pub async fn get_user(google: &Google, access_token: &str) -> Result<GoogleUserInfoResponse, RequestError> {
        let client = reqwest::Client::new();
        let result = client
            .get(google.get_url().get_user_info_url())
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await?;
        match result.status() {
            reqwest::StatusCode::OK => Ok(result.json::<GoogleUserInfoResponse>().await?),
            _ => Err(RequestError::Internal(result.text().await?)),
        }
    }
}
