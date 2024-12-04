use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ExpenseResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}