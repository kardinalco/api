use serde::{Deserialize, Serialize};
use derive_more::Debug;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Google {
    enabled: bool,
    url: GoogleUrl,
    client_id: String,
    #[debug(skip)]
    client_secret: String,
    redirect_uri: String,
    scope: String,
    response_type: String,
    grant_type: String,
    access_type: String,
}

impl Default for Google {
    fn default() -> Self {
        Google {
            enabled: true,
            url: GoogleUrl::default(),
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: String::new(),
            scope: String::from("https://www.googleapis.com/auth/userinfo.email"),
            response_type: String::from("code"),
            grant_type: String::from("authorization_code"),
            access_type: String::from("offline"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GoogleUrl {
    authorize: String,
    token: String,
    userinfo: String,
}

impl GoogleUrl {
    pub fn get_authorize_url(&self) -> &str {
        &self.authorize
    }

    pub fn get_token_url(&self) -> &str {
        &self.token
    }

    pub fn get_user_info_url(&self) -> &str {
        &self.userinfo
    }

    pub fn set_authorize_url(&mut self, authorize: String) {
        self.authorize = authorize;
    }

    pub fn set_token_url(&mut self, token: String) {
        self.token = token;
    }

    pub fn set_user_info_url(&mut self, userinfo: String) {
        self.userinfo = userinfo;
    }
}

impl Default for GoogleUrl {
    fn default() -> Self {
        GoogleUrl {
            authorize: String::from("https://accounts.google.com/o/oauth2/auth"),
            token: String::from("https://accounts.google.com/o/oauth2/token"),
            userinfo: String::from("https://www.googleapis.com/oauth2/v2/userinfo"),
        }
    }
}

impl Google {
    pub fn set_client_id(&mut self, client_id: String) {
        self.client_id = client_id;
    }

    pub fn set_client_secret(&mut self, client_secret: String) {
        //TODO: encrypt
        self.client_secret = client_secret;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_redirect_uri(&mut self, redirect_uri: String) {
        self.redirect_uri = redirect_uri;
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }

    pub fn get_client_secret(&self) -> &str {
        //TODO: decrypt
        &self.client_secret
    }

    pub fn get_grant_type(&self) -> &str {
        &self.grant_type
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_redirect_uri(&self) -> &str {
        &self.redirect_uri
    }

    pub fn build_authorize_url(&self) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&scope={}&response_type={}",
            self.url.authorize, self.client_id, self.redirect_uri, self.scope, self.response_type
        )
    }

    pub fn build_token_url(&self, code: &str) -> String {
        format!(
            "{}?code={}&client_secret={}&client_id={}&grant_type={}",
            self.url.token, code, self.client_secret, self.client_id, self.grant_type
        )
    }

    pub fn get_url(&self) -> &GoogleUrl {
        &self.url
    }
}
