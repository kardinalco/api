use sea_orm_migration::{prelude::*, schema::*};
use serde_json::json;
use crate::extension::postgres::Type;
use crate::m20241016_075756_users::User;
use crate::m20241016_101754_houses::House;
use crate::sea_orm::{DeriveActiveEnum, EnumIter, Iterable};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(Type::create().as_enum(Alias::new("note_type")).values(NoteType::iter()).to_owned()).await?;

        manager.create_table(
            Table::create()
                .table(Note::Table)
                .if_not_exists()
                .col(string_len_uniq(Note::Id, 24).primary_key())
                .col(string_len(Note::Name, 64))
                .col(json_binary(Note::Content).default(json!({})))
                .col(string_len_null(Note::Description, 256))
                .col(enumeration(Note::Type, Alias::new("note_type"), NoteType::iter()).default(NoteType::Flexible))
                .col(date_time(Note::CreatedAt).default("now()"))
                .col(string_len_null(Note::CreatedBy, 24))
                .col(date_time_null(Note::UpdatedAt))
                .col(string_len_null(Note::ArchivedBy, 24))
                .col(date_time_null(Note::ArchivedAt))
                .col(string_len_null(Note::UpdatedBy, 24))
                .col(date_time_null(Note::DeletedAt))
                .col(string_len_null(Note::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_note_created_by").from(Note::Table, Note::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_note_updated_by").from(Note::Table, Note::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_note_deleted_by").from(Note::Table, Note::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_note_archived_by").from(Note::Table, Note::ArchivedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(HouseNote::Table)
                .if_not_exists()
                .primary_key(Index::create().name("pk_house_notes").col(HouseNote::HouseId).col(HouseNote::NoteId))
                .col(string_len(HouseNote::NoteId, 24))
                .col(string_len(HouseNote::HouseId, 24))
                .foreign_key(ForeignKey::create().name("fk_house_note_note_id").from(HouseNote::Table, HouseNote::NoteId).to(Note::Table, Note::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_house_note_house_id").from(HouseNote::Table, HouseNote::HouseId).to(House::Table, House::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Note::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(HouseNote::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum Note {
    Table,
    Id,
    Name,
    Description,
    Content,
    Type,
    CreatedAt,
    CreatedBy,
    ArchivedAt,
    ArchivedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy,
}

#[derive(EnumIter, DeriveActiveEnum, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum")]
enum NoteType {
    #[sea_orm(string_value = "flexible")]
    Flexible,
    #[sea_orm(string_value = "tasks")]
    Tasks,
}

#[derive(DeriveIden)]
enum HouseNote {
    Table,
    NoteId,
    HouseId,
}