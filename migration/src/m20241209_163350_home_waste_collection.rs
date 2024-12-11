use sea_orm_migration::{prelude::*, schema::*};
use serde_json::json;
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
                    .table(HomeWaste::Table)
                    .if_not_exists()
                    .col(string_len(HomeWaste::Id, 24).primary_key())
                    .col(string_len(HomeWaste::HouseId, 24))
                    .col(string_len(HomeWaste::Name, 64))
                    .col(json_binary(HomeWaste::Wastes).default(json!({})))
                    .col(date_time(HomeWaste::CreatedAt).default("now()"))
                    .col(string_len_null(HomeWaste::CreatedBy, 24))
                    .col(date_time_null(HomeWaste::UpdatedAt))
                    .col(string_len_null(HomeWaste::UpdatedBy, 24))
                    .col(date_time_null(HomeWaste::DeletedAt))
                    .col(string_len_null(HomeWaste::DeletedBy, 24))
                    .foreign_key(ForeignKey::create().name("fk_home_waste_house_id").from(HomeWaste::Table, HomeWaste::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_home_waste_created_by").from(HomeWaste::Table, HomeWaste::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_home_waste_updated_by").from(HomeWaste::Table, HomeWaste::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_home_waste_deleted_by").from(HomeWaste::Table, HomeWaste::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(HomeWaste::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum HomeWaste {
    Table,
    Id,
    HouseId,
    Name,
    Wastes,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}
