use entity::sea_orm_active_enums::{ExpenseCurrency, ExpenseFrequency};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ExpenseCreateRequest {
    pub name: String,
    pub description: Option<String>,
    pub frequency: ExpenseFrequency,
    pub amount: f32,
    pub start_date: chrono::NaiveDateTime,
    pub end_date: Option<chrono::NaiveDateTime>,
    pub currency: Currency,
    pub house_id: String,
}

#[derive(Debug, Deserialize)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Into<ExpenseFrequency> for Frequency {
    fn into(self) -> ExpenseFrequency {
        match self {
            Frequency::Daily => ExpenseFrequency::Daily,
            Frequency::Weekly => ExpenseFrequency::Weekly,
            Frequency::Monthly => ExpenseFrequency::Monthly,
            Frequency::Yearly => ExpenseFrequency::Yearly,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ExpenseUpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub frequency: Option<Frequency>,
    pub currency: Option<Currency>,
    pub amount: Option<f64>,
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<Option<chrono::NaiveDateTime>>,
}

#[derive(Debug, Deserialize)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
}

impl Into<ExpenseCurrency> for Currency {
    fn into(self) -> ExpenseCurrency {
        match self {
            Currency::USD => ExpenseCurrency::Usd,
            Currency::EUR => ExpenseCurrency::Eur,
            Currency::GBP => ExpenseCurrency::Gbp,
            Currency::JPY => ExpenseCurrency::Jpy,
        }
    }
}

impl ExpenseCreateRequest {
    pub fn into_model(self, created_by: &str) -> entity::expense::ActiveModel {
        entity::expense::ActiveModel {
            id: Set(cuid2::cuid()),
            name: Set(self.name),
            description: Set(self.description),
            frequency: Set(self.frequency.into()),
            currency: Set(self.currency.into()),
            amount: Set(self.amount),
            start_date: Set(self.start_date),
            end_date: Set(self.end_date),
            created_by: Set(created_by.to_string()),
            ..Default::default()
        }
    }
}