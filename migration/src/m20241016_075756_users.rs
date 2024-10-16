use sea_orm::{EnumIter, Iterable, DeriveActiveEnum};
use sea_orm_migration::{prelude::*, schema::*};
use crate::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Type::create().as_enum(Alias::new("registered_with")).values(RegisteredWith::iter()).to_owned()).await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id))
                    .col(string_len(User::FirstName, 64))
                    .col(string_len_null(User::LastName, 64))
                    .col(string_len_uniq(User::Email, 128))
                    .col(string_len(User::Password, 128))
                    .col(boolean(User::IsActive).default(false))
                    .col(boolean(User::IsDeleted).default(false))
                    .col(string_len_null(User::PhoneNumber, 20))
                    .col(date_null(User::Birthday))
                    .col(string_len_null(User::Country, 64))
                    .col(string_len_null(User::City, 100))
                    .col(string_len_null(User::Address, 100))
                    .col(string_len_null(User::ZipCode, 6))
                    .col(text_null(User::Picture))
                    .col(enumeration(User::RegisteredWith, Alias::new("registered_with"), RegisteredWith::iter()).default(RegisteredWith::Native))
                    .col(date_time(User::CreatedAt).default("now()"))
                    .col(uuid_null(User::CreatedBy))
                    .col(date_time_null(User::UpdatedAt))
                    .col(uuid_null(User::UpdatedBy))
                    .col(date_time_null(User::DeletedAt))
                    .col(uuid_null(User::DeletedBy))
                    .foreign_key(ForeignKey::create().name("fk_created_by_user_id").from(User::Table, User::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_updated_by_user_id").from(User::Table, User::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_deleted_by_user_id").from(User::Table, User::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned()
            ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(enum_name = "payment_options", rs_type = "String", db_type = "Enum")]
enum RegisteredWith {
    #[sea_orm(string_value = "native")]
    Native,
    #[sea_orm(string_value = "google")]
    Google,
    #[sea_orm(string_value = "apple")]
    Apple
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Email,
    FirstName,
    LastName,
    Birthday,
    Password,
    RegisteredWith,
    PhoneNumber,
    City,
    Country,
    ZipCode,
    Address,
    IsDeleted,
    IsActive,
    UpdatedAt,
    UpdatedBy,
    CreatedAt,
    CreatedBy,
    DeletedAt,
    DeletedBy,
    Picture
}