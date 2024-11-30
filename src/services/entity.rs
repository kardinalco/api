use entity::user::{Column, Entity};
use entity::{role, user};
use sea_orm::prelude::Expr;
use sea_orm::sea_query::Alias;
use sea_orm::JoinType::LeftJoin;
use sea_orm::{EntityTrait, QuerySelect, Related, RelationDef, RelationTrait, Select};

pub struct UserCreatedByUser;
pub struct UserUpdatedByUser;
pub struct UserDeletedByUser;

impl Related<Entity> for UserUpdatedByUser {
    fn to() -> RelationDef {
        user::Relation::SelfRef1.def()
    }
}

impl Related<Entity> for UserCreatedByUser {
    fn to() -> RelationDef {
        user::Relation::SelfRef3.def()
    }
}

impl Related<Entity> for UserDeletedByUser {
    fn to() -> RelationDef {
        user::Relation::SelfRef2.def()
    }
}

pub struct RoleCreatedByUser;
pub struct RoleUpdatedByUser;
pub struct RoleDeletedByUser;

impl Related<Entity> for RoleUpdatedByUser {
    fn to() -> RelationDef {
        role::Relation::User1.def()
    }
}

impl Related<Entity> for RoleCreatedByUser {
    fn to() -> RelationDef {
        role::Relation::User2.def()
    }
}

impl Related<Entity> for RoleDeletedByUser {
    fn to() -> RelationDef {
        role::Relation::User3.def()
    }
}

pub trait WithCreatedByUser: Sized {
    fn with_created_user<E: Related<Entity>>(self) -> Self;
}

pub trait WithUpdatedByUser: Sized {
    fn with_updated_user<E: Related<Entity>>(self) -> Self;
}

pub trait WithDeletedByUser: Sized {
    fn with_deleted_user<E: Related<Entity>>(self) -> Self;
}

impl<T: EntityTrait> WithCreatedByUser for Select<T> {
    fn with_created_user<E: Related<Entity>>(self) -> Self {
        //let user_fields: Vec<(Column, &str)> = vec![(Column::FirstName, "created_by_firstname"), (Column::LastName, "created_by_lastname")];
        let alias_created_by = Alias::new("created_by");
        self.join_as(LeftJoin, E::to(), alias_created_by.clone())
            .expr_as(
                Expr::col((alias_created_by.clone(), Column::FirstName)),
                "created_by_firstname",
            )
            .expr_as(
                Expr::col((alias_created_by.clone(), Column::LastName)),
                "created_by_lastname",
            )
            .expr_as(
                Expr::col((alias_created_by.clone(), Column::Email)),
                "created_by_email",
            )
            .expr_as(
                Expr::col((alias_created_by.clone(), Column::DeletedAt)),
                "created_by_deleted_at",
            )
    }
}

impl<T: EntityTrait> WithUpdatedByUser for Select<T> {
    fn with_updated_user<E: Related<Entity>>(self) -> Self {
        let alias_updated_by = Alias::new("updated_by");
        self.join_as(LeftJoin, E::to(), alias_updated_by)
    }
}

impl WithDeletedByUser for Select<user::Entity> {
    fn with_deleted_user<E: Related<Entity>>(self) -> Self {
        let alias_deleted_by = Alias::new("deleted_by");
        self.join_as(LeftJoin, E::to(), alias_deleted_by)
    }
}
