use sea_orm_migration::{prelude::*, schema::*};
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::m20241016_101754_houses::House;
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
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(string_len_uniq(Credentials::Id, 24).primary_key())
                    .col(string_len(Credentials::Name, 64))
                    .col(string_len_null(Credentials::Description, 4096))
                    .col(string_len_null(Credentials::Username, 256))
                    .col(array_null(Credentials::Password, ColumnType::String(StringLen::N(512))))
                    .col(enumeration(Credentials::CredentialsType, Alias::new("credentials_type"), CredentialsAvailableType::iter()).default(CredentialsAvailableType::Other))
                    .col(date_time(Credentials::CreatedAt).default("now()"))
                    .col(string_len_null(Credentials::CreatedBy, 24))
                    .col(date_time_null(Credentials::UpdatedAt))
                    .col(string_len_null(Credentials::UpdatedBy, 24))
                    .col(date_time_null(Credentials::DeletedAt))
                    .col(string_len_null(Credentials::DeletedBy, 24))

                    .foreign_key(ForeignKey::create().name("fk_credentials_created_by_user_id").from(Credentials::Table, Credentials::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_credentials_updated_by_user_id").from(Credentials::Table, Credentials::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_credentials_deleted_by_user_id").from(Credentials::Table, Credentials::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await?;
        
        manager.create_table(
            Table::create()
                .table(CredentialsHouse::Table)
                .primary_key(Index::create().name("pk_credentials_house").col(CredentialsHouse::CredentialsId).col(CredentialsHouse::HouseId))
                .if_not_exists()
                .col(string_len(CredentialsHouse::CredentialsId, 24))
                .col(string_len(CredentialsHouse::HouseId, 24))
                .col(date_time(CredentialsHouse::CreatedAt).default("now()"))
                .col(string_len(CredentialsHouse::CreatedBy, 24))
                .col(date_time_null(CredentialsHouse::UpdatedAt))
                .col(string_len_null(CredentialsHouse::UpdatedBy, 24))
                .col(date_time_null(CredentialsHouse::DeletedAt))
                .col(string_len_null(CredentialsHouse::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_credentials_house_credentials_id").from(CredentialsHouse::Table, CredentialsHouse::CredentialsId).to(Credentials::Table, Credentials::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_credentials_house_house_id").from(CredentialsHouse::Table, CredentialsHouse::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_credentials_house_created_by_user_id").from(CredentialsHouse::Table, CredentialsHouse::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_credentials_house_updated_by_user_id").from(CredentialsHouse::Table, CredentialsHouse::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_credentials_house_deleted_by_user_id").from(CredentialsHouse::Table, CredentialsHouse::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(CredentialsHouse::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Credentials::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("credentials_type")).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Credentials {
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

#[derive(DeriveIden)]
enum CredentialsHouse {
    Table,
    CredentialsId,
    HouseId,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}