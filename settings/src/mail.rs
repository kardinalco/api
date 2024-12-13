use serde::{Deserialize, Serialize};
use derive_more::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Mail {
    listmonk: ListMonk,
    smtp: Smtp,
    from_addr: String,
}

impl Mail {
    pub fn smtp(&self) -> &Smtp {
        &self.smtp
    }

    pub fn listmonk(&self) -> &ListMonk {
        &self.listmonk
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Smtp {
    host: String,
    password: String,
    port: u16,
    tls: SmtpTlsKind,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub enum SmtpTlsKind {
    #[default]
    None,
    StartTls,
    Ssl,
}

impl Default for Smtp {
    fn default() -> Self {
        Smtp {
            port: 587,
            tls: SmtpTlsKind::StartTls,
            password: Default::default(),
            host: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListMonk {
    pub api_user: String,
    pub api_host: String,
    #[debug("*******")]
    pub api_key: String,
    pub templates: ListMonkTemplates,
}

impl Default for ListMonk {
    fn default() -> Self {
        ListMonk {
            api_host: String::from("http://localhost:9030/api"),
            api_user: Default::default(),
            api_key: Default::default(),
            templates: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ListMonkTemplates {
    pub verification: i32,
    pub welcome: i32,
    pub reset_password: i32,
    pub password_changed: i32,
}