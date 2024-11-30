use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241016_075756_users::User;
use crate::m20241016_101754_houses::House;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Provider::Table)
                    .if_not_exists()
                    .col(string_len_uniq(Provider::Id, 24).primary_key())
                    .col(string_len_uniq(Provider::Name, 128))
                    .col(string_len_null(Provider::Description, 4096))
                    .col(date_time(Provider::CreatedAt).default("now()"))
                    .col(string_len(Provider::CreatedBy, 24))
                    .col(date_time_null(Provider::UpdatedAt))
                    .col(string_len_null(Provider::UpdatedBy, 24))
                    .col(date_time_null(Provider::DeletedAt))
                    .col(string_len_null(Provider::DeletedBy, 24))
                    .foreign_key(ForeignKey::create().name("fk_provider_created_by_user_id").from(Provider::Table, Provider::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_provider_updated_by_user_id").from(Provider::Table, Provider::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_provider_deleted_by_user_id").from(Provider::Table, Provider::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await?;
        
        manager.create_table(
            Table::create()
                .table(ProviderHouse::Table)
                .primary_key(Index::create().name("pk_provider_house").col(ProviderHouse::ProviderId).col(ProviderHouse::HouseId))
                .if_not_exists()
                .col(string_len(ProviderHouse::ProviderId, 24))
                .col(string_len(ProviderHouse::HouseId, 24))
                .col(date_time(ProviderHouse::CreatedAt).default("now()"))
                .col(string_len(ProviderHouse::CreatedBy, 24))
                .col(date_time_null(ProviderHouse::UpdatedAt))
                .col(string_len_null(ProviderHouse::UpdatedBy, 24))
                .col(date_time_null(ProviderHouse::DeletedAt))
                .col(string_len_null(ProviderHouse::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_provider_house_provider_id").from(ProviderHouse::Table, ProviderHouse::ProviderId).to(Provider::Table, Provider::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_provider_house_house_id").from(ProviderHouse::Table, ProviderHouse::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_provider_house_created_by_user_id").from(ProviderHouse::Table, ProviderHouse::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_provider_house_updated_by_user_id").from(ProviderHouse::Table, ProviderHouse::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_provider_house_deleted_by_user_id").from(ProviderHouse::Table, ProviderHouse::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(ProviderHouse::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Provider::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Provider {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(DeriveIden)]
enum ProviderHouse {
    Table,
    ProviderId,
    HouseId,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}