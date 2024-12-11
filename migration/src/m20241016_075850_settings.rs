use cuid2::cuid;
use sea_orm_migration::{async_trait, prelude::*, schema::*};
use serde_json::json;
use settings::bucket::Bucket;
use settings::cache::Cache;
use settings::global::Global;
use settings::google::Google;
use settings::mail::Mail;
use crate::m20241016_075756_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Settings::Table)
                .if_not_exists()
                .col(string_len_uniq(Settings::Id, 24).primary_key())
                .col(string_len(Settings::Name, 64))
                .col(string_len(Settings::Description, 256))
                .col(json_binary(Settings::Value))
                .col(date_time(Settings::CreatedAt).default("now()"))
                .col(string_len_null(Settings::CreatedBy, 24))
                .col(date_time_null(Settings::UpdatedAt))
                .col(string_len_null(Settings::UpdatedBy, 24))
                .col(date_time_null(Settings::DeletedAt))
                .col(string_len_null(Settings::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_settings_created_by").from(Settings::Table, Settings::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_settings_updated_by").from(Settings::Table, Settings::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_settings_deleted_by").from(Settings::Table, Settings::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;
        
        let values = vec![
            ("google", "Google settings (credentials, features flipping...)", json!(Google::default())), 
            ("mail", "Mail settings (smtp, from address...)", json!(Mail::default())),
            ("global", "Global settings (pagination, default language...)", json!(Global::default())),
            ("cache", "Cache settings (ttl, refresh...)", json!(Cache::default())),
            ("bucket", "Bucket settings (storage, access...)", json!(json!(Bucket::default()))),
        ];

        let mut google =  Query::insert()
            .into_table(Settings::Table)
            .columns(vec![Settings::Id, Settings::Name, Settings::Description, Settings::Value]).to_owned();
        for (name, description, value) in values {
            google = google.values_panic(vec![cuid().into(), name.to_string().into(),description.to_string().into(), value.into()]).to_owned();
        }
        manager.exec_stmt(google).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Settings::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Settings {
    Table,
    Id,
    Name,
    Description,
    Value,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy
}
