use sea_orm_migration::{prelude::*, schema::*};
use crate::m20241016_101754_houses::House;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Pet::Table)
                    .if_not_exists()
                    .col(string_len(Pet::Id, 24).primary_key())
                    .col(string_len(Pet::Name, 100))
                    .col(date_time(Pet::CreatedAt).default("now()"))
                    .col(string_len(Pet::CreatedBy, 24))
                    .col(date_time_null(Pet::UpdatedAt))
                    .col(string_len_null(Pet::UpdatedBy, 24))
                    .col(date_time_null(Pet::DeletedAt))
                    .col(string_len_null(Pet::DeletedBy, 24))
                    .foreign_key(ForeignKey::create().name("fk_pet_created_by_user_id").from(Pet::Table, Pet::CreatedBy).to(Pet::Table, Pet::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_pet_updated_by_user_id").from(Pet::Table, Pet::UpdatedBy).to(Pet::Table, Pet::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_pet_deleted_by_user_id").from(Pet::Table, Pet::DeletedBy).to(Pet::Table, Pet::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await?;
        
        manager.create_table(
            Table::create()
                .table(PetHouse::Table)
                .primary_key(Index::create().name("pk_pet_house").col(PetHouse::PetId).col(PetHouse::HouseId))
                .if_not_exists()
                .col(string_len(PetHouse::PetId, 24))
                .col(string_len(PetHouse::HouseId, 24))
                .col(date_time(PetHouse::CreatedAt).default("now()"))
                .col(string_len(PetHouse::CreatedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_pet_house_pet_id").from(PetHouse::Table, PetHouse::PetId).to(Pet::Table, Pet::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_pet_house_house_id").from(PetHouse::Table, PetHouse::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_pet_house_created_by_user_id").from(PetHouse::Table, PetHouse::CreatedBy).to(Pet::Table, Pet::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned(),
        ).await?;
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(PetHouse::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Pet::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Pet {
    Table,
    Id,
    Name,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(DeriveIden)]
enum PetHouse {
    Table,
    PetId,
    HouseId,
    CreatedAt,
    CreatedBy,
}