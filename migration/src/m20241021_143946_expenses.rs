use sea_orm_migration::{prelude::*};
use sea_orm_migration::schema::{boolean, date_time, date_time_null, enumeration, float, string_len, string_len_null, string_len_uniq};
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::m20241016_101754_houses::House;
use crate::sea_orm::{DeriveActiveEnum, EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Type::create().as_enum(Alias::new("expense_frequency")).values(ExpenseFrequency::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("expense_currency")).values(ExpenseCurrency::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("payment_method")).values(PaymentMethod::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("payment_status")).values(PaymentStatus::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("reminder_type")).values(ReminderType::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("reminder_status")).values(ReminderStatus::iter()).to_owned()).await?;
        manager.create_type(Type::create().as_enum(Alias::new("shared_type")).values(SharedType::iter()).to_owned()).await?;

        manager.create_table(
            Table::create()
                .table(Expense::Table)
                .if_not_exists()
                .col(string_len_uniq(Expense::Id, 24).primary_key())
                .col(string_len(Expense::Name, 128))
                .col(string_len_null(Expense::Description, 4096))
                .col(float(Expense::Amount))
                .col(enumeration(Expense::Frequency, Alias::new("expense_frequency"), ExpenseFrequency::iter()).default(ExpenseFrequency::Daily))
                .col(enumeration(Expense::Currency, Alias::new("expense_currency"), ExpenseCurrency::iter()).default(ExpenseCurrency::Eur))
                .col(date_time(Expense::StartDate))
                .col(date_time_null(Expense::EndDate))
                .col(boolean(Expense::IsActive).default(true))
                .col(date_time(Expense::CreatedAt).default("now()"))
                .col(string_len(Expense::CreatedBy, 24))
                .col(date_time_null(Expense::UpdatedAt))
                .col(string_len_null(Expense::UpdatedBy, 24))
                .col(date_time_null(Expense::DeletedAt))
                .col(string_len_null(Expense::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_created_by_user_id").from(Expense::Table, Expense::CreatedBy).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_updated_by_user_id").from(Expense::Table, Expense::UpdatedBy).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_deleted_by_user_id").from(Expense::Table, Expense::DeletedBy).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(ExpenseHouse::Table)
                .primary_key(Index::create().name("pk_expense_house").col(ExpenseHouse::ExpenseId).col(ExpenseHouse::HouseId))
                .if_not_exists()
                .col(string_len(ExpenseHouse::ExpenseId, 24))
                .col(string_len(ExpenseHouse::HouseId, 24))
                .col(date_time(ExpenseHouse::CreatedAt).default("now()"))
                .col(string_len(ExpenseHouse::CreatedBy, 24))
                .col(date_time_null(ExpenseHouse::UpdatedAt))
                .col(string_len_null(ExpenseHouse::UpdatedBy, 24))
                .col(date_time_null(ExpenseHouse::DeletedAt))
                .col(string_len_null(ExpenseHouse::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_house_expense_id").from(ExpenseHouse::Table, ExpenseHouse::ExpenseId).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_house_house_id").from(ExpenseHouse::Table, ExpenseHouse::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_house_created_by_user_id").from(ExpenseHouse::Table, ExpenseHouse::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_house_updated_by_user_id").from(ExpenseHouse::Table, ExpenseHouse::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_house_deleted_by_user_id").from(ExpenseHouse::Table, ExpenseHouse::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(ExpensePayment::Table)
                .if_not_exists()
                .col(string_len_uniq(ExpensePayment::Id, 24).primary_key())
                .col(string_len(ExpensePayment::ExpenseId, 24))
                .col(float(ExpensePayment::Amount))
                .col(date_time(ExpensePayment::PaymentDate))
                .col(enumeration(ExpensePayment::PaymentMethod, Alias::new("payment_method"), PaymentMethod::iter()).default(PaymentMethod::Cash))
                .col(enumeration(ExpensePayment::Status, Alias::new("payment_status"), PaymentStatus::iter()).default(PaymentStatus::Pending))
                .col(date_time(ExpensePayment::CreatedAt).default("now()"))
                .col(string_len(ExpensePayment::CreatedBy, 24))
                .col(date_time_null(ExpensePayment::UpdatedAt))
                .col(string_len_null(ExpensePayment::UpdatedBy, 24))
                .col(date_time_null(ExpensePayment::DeletedAt))
                .col(string_len_null(ExpensePayment::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_payment_expense_id").from(ExpensePayment::Table, ExpensePayment::ExpenseId).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_payment_created_by_user_id").from(ExpensePayment::Table, ExpensePayment::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_payment_updated_by_user_id").from(ExpensePayment::Table, ExpensePayment::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_payment_deleted_by_user_id").from(ExpensePayment::Table, ExpensePayment::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(ExpenseReminder::Table)
                .if_not_exists()
                .col(string_len_uniq(ExpenseReminder::Id, 24).primary_key())
                .col(string_len(ExpenseReminder::ExpenseId, 24))
                .col(date_time(ExpenseReminder::ReminderDate))
                .col(string_len(ExpenseReminder::ReminderTime, 8))
                .col(enumeration(ExpenseReminder::ReminderType, Alias::new("reminder_type"), ReminderType::iter()).default(ReminderType::Email))
                .col(enumeration(ExpenseReminder::Status, Alias::new("reminder_status"), ReminderStatus::iter()).default(ReminderStatus::Pending))
                .col(date_time(ExpenseReminder::CreatedAt).default("now()"))
                .col(string_len(ExpenseReminder::CreatedBy, 24))
                .col(date_time_null(ExpenseReminder::UpdatedAt))
                .col(string_len_null(ExpenseReminder::UpdatedBy, 24))
                .col(date_time_null(ExpenseReminder::DeletedAt))
                .col(string_len_null(ExpenseReminder::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_reminder_expense_id").from(ExpenseReminder::Table, ExpenseReminder::ExpenseId).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_reminder_created_by_user_id").from(ExpenseReminder::Table, ExpenseReminder::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_reminder_updated_by_user_id").from(ExpenseReminder::Table, ExpenseReminder::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_reminder_deleted_by_user_id").from(ExpenseReminder::Table, ExpenseReminder::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;

        manager.create_table(
            Table::create()
                .table(ExpenseShared::Table)
                .if_not_exists()
                .col(string_len_uniq(ExpenseShared::Id, 24).primary_key())
                .col(string_len(ExpenseShared::ExpenseId, 24))
                .col(string_len(ExpenseShared::UserId, 24))
                .col(enumeration(ExpenseShared::Type, Alias::new("shared_type"), SharedType::iter()).default(SharedType::Percentage))
                .col(float(ExpenseShared::Value))
                .col(date_time(ExpenseShared::CreatedAt).default("now()"))
                .col(string_len(ExpenseShared::CreatedBy, 24))
                .col(date_time_null(ExpenseShared::UpdatedAt))
                .col(string_len_null(ExpenseShared::UpdatedBy, 24))
                .col(date_time_null(ExpenseShared::DeletedAt))
                .col(string_len_null(ExpenseShared::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_shared_expense_id").from(ExpenseShared::Table, ExpenseShared::ExpenseId).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_shared_user_id").from(ExpenseShared::Table, ExpenseShared::UserId).to(User::Table, User::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_shared_created_by_user_id").from(ExpenseShared::Table, ExpenseShared::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_shared_updated_by_user_id").from(ExpenseShared::Table, ExpenseShared::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_shared_deleted_by_user_id").from(ExpenseShared::Table, ExpenseShared::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(ExpenseReimbursement::Table)
                .if_not_exists()
                .col(string_len_uniq(ExpenseReimbursement::Id, 24).primary_key())
                .col(string_len(ExpenseReimbursement::ExpenseId, 24))
                .col(float(ExpenseReimbursement::Amount))
                .col(date_time(ExpenseReimbursement::ReimbursementDate))
                .col(enumeration(ExpenseReimbursement::Status, Alias::new("payment_status"), PaymentStatus::iter()).default(PaymentStatus::Pending))
                .col(date_time(ExpenseReimbursement::CreatedAt).default("now()"))
                .col(string_len(ExpenseReimbursement::CreatedBy, 24))
                .col(date_time_null(ExpenseReimbursement::UpdatedAt))
                .col(string_len_null(ExpenseReimbursement::UpdatedBy, 24))
                .col(date_time_null(ExpenseReimbursement::DeletedAt))
                .col(string_len_null(ExpenseReimbursement::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_expense_refund_expense_id").from(ExpenseReimbursement::Table, ExpenseReimbursement::ExpenseId).to(Expense::Table, Expense::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_refund_created_by_user_id").from(ExpenseReimbursement::Table, ExpenseReimbursement::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_refund_updated_by_user_id").from(ExpenseReimbursement::Table, ExpenseReimbursement::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_expense_refund_deleted_by_user_id").from(ExpenseReimbursement::Table, ExpenseReimbursement::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ExpenseReimbursement::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpenseShared::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpenseReminder::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpensePayment::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(ExpenseHouse::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Expense::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("expense_frequency")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("expense_currency")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("payment_method")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("payment_status")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("reminder_type")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("reminder_status")).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("shared_type")).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Expense {
    Table,
    Id,
    Name,
    Amount,
    Frequency,
    StartDate,
    EndDate,
    IsActive,
    Currency,
    Description,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum ExpenseFrequency {
    #[sea_orm(string_value = "daily")]
    Daily,
    #[sea_orm(string_value = "weekly")]
    Weekly,
    #[sea_orm(string_value = "monthly")]
    Monthly,
    #[sea_orm(string_value = "yearly")]
    Yearly,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum ExpenseCurrency {
    #[sea_orm(string_value = "usd")]
    Usd,
    #[sea_orm(string_value = "eur")]
    Eur,
    #[sea_orm(string_value = "gbp")]
    Gbp,
    #[sea_orm(string_value = "jpy")]
    Jpy,
    #[sea_orm(string_value = "cny")]
    Cny,
}

#[derive(DeriveIden)]
enum ExpensePayment {
    Table,
    Id,
    ExpenseId,
    Amount,
    PaymentDate,
    PaymentMethod,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum PaymentMethod {
    #[sea_orm(string_value = "cash")]
    Cash,
    #[sea_orm(string_value = "credit_card")]
    Card,
    #[sea_orm(string_value = "bank_transfer")]
    BankTransfer,
    #[sea_orm(string_value = "check")]
    Check,
    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum PaymentStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "paid")]
    Paid,
    #[sea_orm(string_value = "overdue")]
    Overdue,
}

#[derive(DeriveIden)]
enum ExpenseReminder {
    Table,
    Id,
    ExpenseId,
    ReminderDate,
    ReminderTime,
    ReminderType,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum ReminderType {
    #[sea_orm(string_value = "email")]
    Email,
    #[sea_orm(string_value = "sms")]
    Sms,
    #[sea_orm(string_value = "notification")]
    Notification,
    #[sea_orm(string_value = "other")]
    Other,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum ReminderStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "sent")]
    Sent,
    #[sea_orm(string_value = "failed")]
    Failed,
}

#[derive(DeriveIden)]
enum ExpenseShared {
    Table,
    Id,
    ExpenseId,
    UserId,
    Type,
    Value,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum SharedType {
    #[sea_orm(string_value = "percentage")]
    Percentage,
    #[sea_orm(string_value = "fixed")]
    Fixed,
}

#[derive(DeriveIden)]
enum ExpenseReimbursement {
    Table,
    Id,
    ExpenseId,
    Amount,
    ReimbursementDate,
    Status,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}


#[derive(DeriveIden)]
enum ExpenseHouse {
    Table,
    ExpenseId,
    HouseId,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}