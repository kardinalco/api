use sea_orm_migration::{prelude::*, schema::*};
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::sea_orm::{DeriveActiveEnum, EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Type::create().as_enum(Alias::new("house_location_type")).values(HouseLocationType::iter()).to_owned()).await?;
        // House Table
        manager
            .create_table(
                Table::create()
                    .table(House::Table)
                    .if_not_exists()
                    .col(string_len_uniq(House::Id, 24).primary_key())
                    .col(string_len(House::Name, 64))
                    .col(string_len(House::Code, 24))
                    .col(string_len_null(House::Description, 4096))
                    .col(string_len_null(House::City, 64))
                    .col(string_len_null(House::Country, 64))
                    .col(string_len_null(House::ZipCode, 24))
                    .col(string_len_null(House::Address, 256))
                    .col(float(House::Surface).default(0.0))
                    .col(integer_null(House::Rooms))
                    .col(float_null(House::Latitude))
                    .col(float_null(House::Longitude))
                    .col(enumeration(House::LocationType, Alias::new("house_location_type"), HouseLocationType::iter()).default(HouseLocationType::Unknown))
                    .col(string_len_null(House::OwnerName, 64))
                    .col(string_len_null(House::OwnerContactInfo, 64))
                    .col(string_len_null(House::OwnerPhone, 24))
                    .col(string_len_null(House::OwnerEmail, 64))
                    .col(integer_null(House::BuiltYear))
                    .col(date_time_null(House::AcquiredAt))
                    .col(date_time(House::CreatedAt).default("now()"))
                    .col(string_len(House::CreatedBy, 24))
                    .col(date_time_null(House::UpdatedAt))
                    .col(string_len_null(House::UpdatedBy, 24))
                    .col(date_time_null(House::DeletedAt))
                    .col(string_len_null(House::DeletedBy, 24))
                    .foreign_key(ForeignKey::create().name("fk_house_created_by_user_id").from(House::Table, House::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_house_updated_by_user_id").from(House::Table, House::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .foreign_key(ForeignKey::create().name("fk_house_deleted_by_user_id").from(House::Table, House::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                    .to_owned(),
            )
            .await?;

        // HouseUserStatus Enum Type
        manager.create_type(Type::create().as_enum(Alias::new("house_user_status")).values(HouseUserStatus::iter()).to_owned()).await?;

        // HouseUser Table
        manager.create_table(
            Table::create()
                .table(HouseUser::Table)
                .primary_key(Index::create().name("pk_house_user").col(HouseUser::UserId).col(HouseUser::HouseId))
                .if_not_exists()
                .col(string_len(HouseUser::UserId, 24)).foreign_key(ForeignKey::create().name("fk_house_user_user_id").from(HouseUser::Table, HouseUser::UserId).to(User::Table, User::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(string_len(HouseUser::HouseId, 24)).foreign_key(ForeignKey::create().name("fk_house_user_house_id").from(HouseUser::Table, HouseUser::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(date_time(HouseUser::InvitedAt).default("now()"))
                .col(date_time_null(HouseUser::AcceptedAt))
                .col(date_time_null(HouseUser::DeclinedAt))
                .col(enumeration(HouseUser::Status, Alias::new("house_user_status"), HouseUserStatus::iter()).default(HouseUserStatus::Pending))
                .to_owned()
        ).await?;
        Ok(())

    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(House::Table).to_owned()).await?;
        manager.drop_type(Type::drop().name(Alias::new("house_user_status")).to_owned()).await?;
        manager.drop_table(Table::drop().table(HouseUser::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
pub enum House {
    Table,
    Id,
    Name,
    Code,
    Description,
    City,
    Country,
    ZipCode,
    Address,
    Latitude,
    Longitude,
    LocationType,
    Rooms,
    OwnerName,
    OwnerContactInfo,
    OwnerPhone,
    OwnerEmail,
    BuiltYear,
    AcquiredAt,
    Surface,
    UpdatedAt,
    UpdatedBy,
    CreatedAt,
    CreatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum HouseLocationType {
    #[sea_orm(string_value = "owned")]
    Owned,
    #[sea_orm(string_value = "rented")]
    Rented,
    #[sea_orm(string_value = "unknown")]
    Unknown
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum HouseUserStatus {
    #[sea_orm(string_value = "pending")]
    Pending,
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "declined")]
    Declined
}

#[derive(DeriveIden)]
pub enum HouseUser {
    Table,
    UserId,
    HouseId,
    InvitedAt,
    AcceptedAt,
    DeclinedAt,
    Status,
}