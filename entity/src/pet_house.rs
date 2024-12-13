//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.2

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "pet_house")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub pet_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub house_id: String,
    pub created_at: DateTime,
    pub created_by: String,
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
        belongs_to = "super::pet::Entity",
        from = "Column::CreatedBy",
        to = "super::pet::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Pet2,
    #[sea_orm(
        belongs_to = "super::pet::Entity",
        from = "Column::PetId",
        to = "super::pet::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Pet1,
}

impl Related<super::house::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::House.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
