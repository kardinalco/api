use sea_orm_migration::{async_trait, prelude::*, schema::*};
use cuid2::cuid;
use crate::m20241016_075756_users::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
            Table::create()
                .table(Role::Table)
                .if_not_exists()
                .col(string_len_uniq(Role::Id, 24).primary_key())
                .col(string_len(Role::Name, 64))
                .col(string_len_null(Role::Description, 256))
                .col(date_time(Role::CreatedAt).default("now()"))
                .col(string_len_null(Role::CreatedBy, 24))
                .col(date_time_null(Role::UpdatedAt))
                .col(string_len_null(Role::UpdatedBy, 24))
                .col(date_time_null(Role::DeletedAt))
                .col(string_len_null(Role::DeletedBy, 24))
                .foreign_key(ForeignKey::create().name("fk_created_by").from(Role::Table, Role::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_updated_by").from(Role::Table, Role::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_deleted_by").from(Role::Table, Role::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(Permission::Table)
                .if_not_exists()
                .col(string_len_uniq(Permission::Id, 24).primary_key())
                .col(string_len(Permission::Action, 64))
                .col(string_len(Permission::Resource, 64))
                .col(string_len_null(Permission::Description, 256))
                .col(date_time(Permission::CreatedAt).default("now()"))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(RolePermission::Table)
                .if_not_exists()
                .col(string_len(RolePermission::RoleId, 24)).foreign_key(ForeignKey::create().name("fk_role_id").from(RolePermission::Table, RolePermission::RoleId).to(Role::Table, Role::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(string_len(RolePermission::PermissionId, 24)).foreign_key(ForeignKey::create().name("fk_permission_id").from(RolePermission::Table, RolePermission::PermissionId).to(Permission::Table, Permission::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(date_time(RolePermission::CreatedAt).default("now()"))
                .primary_key(Index::create().name("pk_role_permission").col(RolePermission::RoleId).col(RolePermission::PermissionId))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(UserRole::Table)
                .if_not_exists()
                .col(string_len(UserRole::UserId, 24)).foreign_key(ForeignKey::create().name("fk_user_id").from(UserRole::Table, UserRole::UserId).to(User::Table, User::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(string_len(UserRole::RoleId, 24)).foreign_key(ForeignKey::create().name("fk_role_id").from(UserRole::Table, UserRole::RoleId).to(Role::Table, Role::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(date_time(UserRole::CreatedAt).default("now()"))
                .primary_key(Index::create().name("pk_user_role").col(UserRole::UserId).col(UserRole::RoleId))
                .to_owned()
        ).await?;

        let roles = Query::insert()
            .into_table(Role::Table)
            .columns(vec![Role::Id, Role::Name, Role::Description])
            .values_panic([cuid().into(), "Admin".into(), "Admin system role".into()])
            .values_panic([cuid().into(), "User".into(), "User system role".into()])
            .to_owned();
        manager.exec_stmt(roles).await?;

        let user_permissions = vec![
            ("read_self", "user", "Permission de lire son propre profil"),
            ("update_self", "user", "Permission de modifier son propre profil"),
            ("delete_self", "user", "Permission de supprimer son propre profil"),
            ("read", "user", "Permission de lire des profils"),
            ("update", "user", "Permission de modifier des profils"),
            ("delete", "user", "Permission de supprimer des profils"),
            ("create", "user", "Permission de créer des profils"),
            ("read", "role", "Permission de lire des rôles"),
            ("update", "role", "Permission de modifier des rôles"),
            ("delete", "role", "Permission de supprimer des rôles"),
            ("create", "role", "Permission de créer des rôles"),
            ("read", "permission", "Permission de lire des permissions"),
            ("update", "permission", "Permission de modifier des permissions"),
            ("delete", "permission", "Permission de supprimer des permissions"),
            ("create", "permission", "Permission de créer des permissions"),
        ];
        let mut permissions = Query::insert()
            .into_table(Permission::Table)
            .columns(vec![Permission::Id, Permission::Action, Permission::Resource, Permission::Description]).to_owned();
        for (action, resource, description) in user_permissions {
            permissions = permissions.values_panic([cuid().into(), action.into(), resource.into(), description.into()]).to_owned();
        }
        manager.exec_stmt(permissions.to_owned()).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(UserRole::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(RolePermission::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Permission::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Role::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Role {
    Table,
    Id,
    Name,
    Description,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
    DeletedAt,
    DeletedBy
}

#[derive(DeriveIden)]
pub enum Permission {
    Table,
    Id,
    Action,
    Resource,
    Description,
    CreatedAt
}

#[derive(DeriveIden)]
pub enum RolePermission {
    Table,
    RoleId,
    PermissionId,
    CreatedAt
}

#[derive(DeriveIden)]
pub enum UserRole {
    Table,
    UserId,
    RoleId,
    CreatedAt
}