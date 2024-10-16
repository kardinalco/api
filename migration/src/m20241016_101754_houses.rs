use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241016_075756_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(House::Table)
                    .if_not_exists()
                    .col(pk_auto(House::Id))
                    .col(string_len(House::Name, 64))
                    .col(string_len_null(House::Description, 4096))
                    .col(date_time(House::CreatedAt).default("now()"))
                    .col(uuid_null(House::CreatedBy))
                    .col(date_time_null(House::UpdatedAt))
                    .col(uuid_null(House::UpdatedBy))
                    .col(date_time_null(House::DeletedAt))
                    .col(uuid_null(House::DeletedBy))
                    .foreign_key(ForeignKey::create().name("fk_created_by_user_id").from(House::Table, House::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_updated_by_user_id").from(House::Table, House::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_deleted_by_user_id").from(House::Table, House::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(House::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum House {
    Table,
    Id,
    Name,
    Description,
    City,
    Country,
    ZipCode,
    Address,
    Latitude,
    Longitude,
    UpdatedAt,
    UpdatedBy,
    CreatedAt,
    CreatedBy,
    DeletedAt,
    DeletedBy,
}
