use sea_orm_migration::{prelude::*, schema::*};
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::sea_orm::{DeriveActiveEnum, EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Type::create().as_enum(Alias::new("credentials_type")).values(CredentialsAvailableType::iter()).to_owned()).await?;
        manager
            .create_table(
                Table::create()
                    .table(Password::Table)
                    .if_not_exists()
                    .col(string_len_uniq(Password::Id, 24).primary_key())
                    .col(string_len(Password::Name, 64))
                    .col(string_len_null(Password::Description, 4096))
                    .col(string_len_null(Password::Username, 256))
                    .col(string_len_null(Password::Password, 512))
                    .col(enumeration(Password::CredentialsType, Alias::new("credentials_type"), CredentialsAvailableType::iter()).default(CredentialsAvailableType::Other))
                    .col(date_time(Password::CreatedAt).default("now()"))
                    .col(string_len_null(Password::CreatedBy, 24))
                    .col(date_time_null(Password::UpdatedAt))
                    .col(string_len_null(Password::UpdatedBy, 24))
                    .col(date_time_null(Password::DeletedAt))
                    .col(string_len_null(Password::DeletedBy, 24))

                    .foreign_key(ForeignKey::create().name("fk_created_by_user_id").from(Password::Table, Password::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_updated_by_user_id").from(Password::Table, Password::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_deleted_by_user_id").from(Password::Table, Password::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Password::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Password {
    Table,
    Id,
    Name,
    Description,
    Username,
    Password,
    CredentialsType,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum CredentialsAvailableType {
    #[sea_orm(string_value = "wifi")]
    Wifi,
    #[sea_orm(string_value = "email")]
    Email,
    #[sea_orm(string_value = "cloud_storage")]
    CloudStorage,
    #[sea_orm(string_value = "application")]
    Application,
    #[sea_orm(string_value = "device")]
    Device,
    #[sea_orm(string_value = "database")]
    Database,
    #[sea_orm(string_value = "streaming_service")]
    StreamingService,
    #[sea_orm(string_value = "bank_account")]
    BankAccount,
    #[sea_orm(string_value = "website")]
    Website,
    #[sea_orm(string_value = "other")]
    Other,
}
