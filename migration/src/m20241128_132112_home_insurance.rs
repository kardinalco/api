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
                    .table(HomeInsurance::Table)
                    .col(string_len(HomeInsurance::Id, 24).primary_key())
                    .col(string_len(HomeInsurance::Name, 100))
                    .col(text(HomeInsurance::Description))
                    .col(date_time(HomeInsurance::CreatedAt).default("now()"))
                    .col(string_len(HomeInsurance::CreatedBy, 24))
                    .col(date_time_null(HomeInsurance::UpdatedAt))
                    .col(string_len_null(HomeInsurance::UpdatedBy, 24))
                    .col(date_time_null(HomeInsurance::DeletedAt))
                    .col(string_len_null(HomeInsurance::DeletedBy, 24))
                    .foreign_key(ForeignKey::create().name("fk_home_insurance_created_by_user_id").from(HomeInsurance::Table, HomeInsurance::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_home_insurance_updated_by_user_id").from(HomeInsurance::Table, HomeInsurance::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_home_insurance_deleted_by_user_id").from(HomeInsurance::Table, HomeInsurance::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        manager.create_table(
            Table::create()
                .table(HomeInsuranceHouse::Table)
                .primary_key(Index::create().name("pk_home_insurance_house").col(HomeInsuranceHouse::HomeInsuranceId).col(HomeInsuranceHouse::HouseId))
                .col(string_len(HomeInsuranceHouse::HomeInsuranceId, 24))
                .col(string_len(HomeInsuranceHouse::HouseId, 24))
                .col(date_time(HomeInsuranceHouse::CreatedAt).default("now()"))
                .col(string_len(HomeInsuranceHouse::CreatedBy, 24))
                .col(date_time_null(HomeInsuranceHouse::UpdatedAt))
                .col(string_len_null(HomeInsuranceHouse::UpdatedBy, 24))
                .col(date_time_null(HomeInsuranceHouse::DeletedAt))
                .col(string_len_null(HomeInsuranceHouse::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_home_insurance_house_home_insurance_id").from(HomeInsuranceHouse::Table, HomeInsuranceHouse::HomeInsuranceId).to(HomeInsurance::Table, HomeInsurance::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_home_insurance_house_house_id").from(HomeInsuranceHouse::Table, HomeInsuranceHouse::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_home_insurance_house_created_by_user_id").from(HomeInsuranceHouse::Table, HomeInsuranceHouse::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_home_insurance_house_updated_by_user_id").from(HomeInsuranceHouse::Table, HomeInsuranceHouse::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_home_insurance_house_deleted_by_user_id").from(HomeInsuranceHouse::Table, HomeInsuranceHouse::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .if_not_exists()
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(HomeInsuranceHouse::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(HomeInsurance::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum HomeInsurance {
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
enum HomeInsuranceHouse {
    Table,
    HomeInsuranceId,
    HouseId,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}