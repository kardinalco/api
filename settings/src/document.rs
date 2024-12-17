use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Document {
    pub gcu: DocumentInfo,
    pub privacy_policy: DocumentInfo,
    pub changelog: Vec<DocumentInfo>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DocumentInfo {
    pub content: String,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
    pub kind: DocumentKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DocumentKind {
    GCU,
    PrivacyPolicy,
    TermsOfService,
    ChangelogVersion,
}