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
                .table(HouseHomeInsurance::Table)
                .primary_key(Index::create().name("pk_house_home_insurance").col(HouseHomeInsurance::HomeInsuranceId).col(HouseHomeInsurance::HouseId))
                .col(string_len(HouseHomeInsurance::HomeInsuranceId, 24))
                .col(string_len(HouseHomeInsurance::HouseId, 24))
                .col(date_time(HouseHomeInsurance::CreatedAt).default("now()"))
                .col(string_len(HouseHomeInsurance::CreatedBy, 24))
                .col(date_time_null(HouseHomeInsurance::UpdatedAt))
                .col(string_len_null(HouseHomeInsurance::UpdatedBy, 24))
                .col(date_time_null(HouseHomeInsurance::DeletedAt))
                .col(string_len_null(HouseHomeInsurance::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_house_home_insurance_home_insurance_id").from(HouseHomeInsurance::Table, HouseHomeInsurance::HomeInsuranceId).to(HomeInsurance::Table, HomeInsurance::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_house_home_insurance_house_id").from(HouseHomeInsurance::Table, HouseHomeInsurance::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_house_home_insurance_created_by_user_id").from(HouseHomeInsurance::Table, HouseHomeInsurance::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_house_home_insurance_updated_by_user_id").from(HouseHomeInsurance::Table, HouseHomeInsurance::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_house_home_insurance_deleted_by_user_id").from(HouseHomeInsurance::Table, HouseHomeInsurance::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .if_not_exists()
                .to_owned(),
        ).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(HouseHomeInsurance::Table).to_owned()).await?;
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
enum HouseHomeInsurance {
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