use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}