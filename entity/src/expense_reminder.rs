//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use super::sea_orm_active_enums::ReminderStatus;
use super::sea_orm_active_enums::ReminderType;
use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "expense_reminder")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub expense_id: String,
    pub reminder_date: DateTime,
    pub reminder_time: String,
    pub reminder_type: ReminderType,
    pub status: ReminderStatus,
    pub created_at: DateTime,
    pub created_by: String,
    pub updated_at: Option<DateTime>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime>,
    pub deleted_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::expense::Entity",
        from = "Column::ExpenseId",
        to = "super::expense::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Expense,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::CreatedBy",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User3,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::DeletedBy",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User2,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UpdatedBy",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    User1,
}

impl Related<super::expense::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Expense.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
