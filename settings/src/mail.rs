use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mail {
    smtp: Smtp,
    from_addr: String,
}

impl Default for Mail {
    fn default() -> Self {
        Mail {
            smtp: Smtp::default(),
            from_addr: String::from(""),
        }
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
            host: String::from(""),
            password: String::from(""),
            tls: SmtpTlsKind::default(),
            port: 587,
        }
    }
}
