//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "house_note")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub note_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub house_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::house::Entity",
        from = "Column::HouseId",
        to = "super::house::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    House,
    #[sea_orm(
        belongs_to = "super::note::Entity",
        from = "Column::NoteId",
        to = "super::note::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Note,
}

impl Related<super::house::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::House.def()
    }
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
