//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "home_insurance_house")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub home_insurance_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub house_id: String,
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
        belongs_to = "super::home_insurance::Entity",
        from = "Column::HomeInsuranceId",
        to = "super::home_insurance::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    HomeInsurance,
    #[sea_orm(
        belongs_to = "super::house::Entity",
        from = "Column::HouseId",
        to = "super::house::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    House,
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

impl Related<super::home_insurance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::HomeInsurance.def()
    }
}

impl Related<super::house::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::House.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
