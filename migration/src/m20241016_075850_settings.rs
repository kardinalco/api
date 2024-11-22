use sea_orm_migration::{async_trait, prelude::*, schema::*};
use sea_orm_migration::sea_orm::Iterable;
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::sea_orm::{DeriveActiveEnum, EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let settings_type_alias = Alias::new("settings_type");
        manager.create_type(Type::create().as_enum(settings_type_alias.clone()).values(SettingsType::iter()).to_owned()).await?;

        manager.create_table(
            Table::create()
                .table(Settings::Table)
                .if_not_exists()
                .col(string_len_uniq(Settings::Id, 24).primary_key())
                .col(string_len(Settings::Name, 64))
                .col(string_len(Settings::Description, 256))
                .col(json(Settings::Value))
                .col(enumeration(Settings::Type, settings_type_alias, SettingsType::iter()))
                .col(date_time(Settings::CreatedAt).default("now()"))
                .col(string_len_null(Settings::CreatedBy, 24))
                .col(date_time_null(Settings::UpdatedAt))
                .col(string_len_null(Settings::UpdatedBy, 24))
                .col(date_time_null(Settings::DeletedAt))
                .col(string_len_null(Settings::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_created_by").from(Settings::Table, Settings::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_updated_by").from(Settings::Table, Settings::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_deleted_by").from(Settings::Table, Settings::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Settings::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
pub enum SettingsType {
    #[sea_orm(string_value = "string")]
    String,
    #[sea_orm(string_value = "secret")]
    Secret,
    #[sea_orm(string_value = "number")]
    Number,
    #[sea_orm(string_value = "boolean")]
    Boolean,
    #[sea_orm(string_value = "string_array")]
    StringArray,
    #[sea_orm(string_value = "boolean_array")]
    BooleanArray,
}

#[derive(DeriveIden)]
pub enum Settings {
    Table,
    Id,
    Name,
    Description,
    Value,
    Type,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy
}
