//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "credentials_type")]
pub enum CredentialsType {
    #[sea_orm(string_value = "application")]
    Application,
    #[sea_orm(string_value = "bank_account")]
    BankAccount,
    #[sea_orm(string_value = "cloud_storage")]
    CloudStorage,
    #[sea_orm(string_value = "database")]
    Database,
    #[sea_orm(string_value = "device")]
    Device,
    #[sea_orm(string_value = "email")]
    Email,
    #[sea_orm(string_value = "other")]
    Other,
    #[sea_orm(string_value = "streaming_service")]
    StreamingService,
    #[sea_orm(string_value = "website")]
    Website,
    #[sea_orm(string_value = "wifi")]
    Wifi,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "expense_currency")]
pub enum ExpenseCurrency {
    #[sea_orm(string_value = "cny")]
    Cny,
    #[sea_orm(string_value = "eur")]
    Eur,
    #[sea_orm(string_value = "gbp")]
    Gbp,
    #[sea_orm(string_value = "jpy")]
    Jpy,
    #[sea_orm(string_value = "usd")]
    Usd,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "expense_frequency")]
pub enum ExpenseFrequency {
    #[sea_orm(string_value = "daily")]
    Daily,
    #[sea_orm(string_value = "monthly")]
    Monthly,
    #[sea_orm(string_value = "weekly")]
    Weekly,
    #[sea_orm(string_value = "yearly")]
    Yearly,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "house_location_type"
)]
pub enum HouseLocationType {
    #[sea_orm(string_value = "owned")]
    Owned,
    #[sea_orm(string_value = "rented")]
    Rented,
    #[sea_orm(string_value = "unknown")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "house_user_status")]
pub enum HouseUserStatus {
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "declined")]
    Declined,
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "revoked")]
    Revoked,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "note_type")]
pub enum NoteType {
    #[sea_orm(string_value = "flexible")]
    Flexible,
    #[sea_orm(string_value = "tasks")]
    Tasks,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_method")]
pub enum PaymentMethod {
    #[sea_orm(string_value = "bank_transfer")]
    BankTransfer,
    #[sea_orm(string_value = "card")]
    Card,
    #[sea_orm(string_value = "cash")]
    Cash,
    #[sea_orm(string_value = "check")]
    Check,
    #[sea_orm(string_value = "other")]
    Other,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_status")]
pub enum PaymentStatus {
    #[sea_orm(string_value = "overdue")]
    Overdue,
    #[sea_orm(string_value = "paid")]
    Paid,
    #[sea_orm(string_value = "pending")]
    Pending,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "registered_with")]
pub enum RegisteredWith {
    #[sea_orm(string_value = "apple")]
    Apple,
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "native")]
    Native,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "reminder_status")]
pub enum ReminderStatus {
    #[sea_orm(string_value = "failed")]
    Failed,
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "sent")]
    Sent,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "reminder_type")]
pub enum ReminderType {
    #[sea_orm(string_value = "email")]
    Email,
    #[sea_orm(string_value = "notification")]
    Notification,
    #[sea_orm(string_value = "other")]
    Other,
    #[sea_orm(string_value = "sms")]
    Sms,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "shared_type")]
pub enum SharedType {
    #[sea_orm(string_value = "fixed")]
    Fixed,
    #[sea_orm(string_value = "percentage")]
    Percentage,
}
