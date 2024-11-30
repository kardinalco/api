use sea_orm_migration::{async_trait, prelude::*, schema::*};
use cuid2::cuid;
use permission::resource::Resource;
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
                .foreign_key(ForeignKey::create().name("fk_permission_created_by").from(Role::Table, Role::CreatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_permission_updated_by").from(Role::Table, Role::UpdatedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
                .foreign_key(ForeignKey::create().name("fk_permission_deleted_by").from(Role::Table, Role::DeletedBy).to(User::Table, User::Id).on_delete(ForeignKeyAction::SetNull).on_update(ForeignKeyAction::Cascade))
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
                .primary_key(Index::create().name("pk_role_permission").col(RolePermission::RoleId).col(RolePermission::PermissionId))
                .col(string_len(RolePermission::RoleId, 24)).foreign_key(ForeignKey::create().name("fk_role_permission_role_id").from(RolePermission::Table, RolePermission::RoleId).to(Role::Table, Role::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(string_len(RolePermission::PermissionId, 24)).foreign_key(ForeignKey::create().name("fk_role_permission_permission_id").from(RolePermission::Table, RolePermission::PermissionId).to(Permission::Table, Permission::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(date_time(RolePermission::CreatedAt).default("now()"))
                .to_owned()
        ).await?;

        manager.create_table(
            Table::create()
                .table(UserRole::Table)
                .if_not_exists()
                .primary_key(Index::create().name("pk_user_role").col(UserRole::UserId).col(UserRole::RoleId))
                .col(string_len(UserRole::UserId, 24)).foreign_key(ForeignKey::create().name("fk_user_role_user_id").from(UserRole::Table, UserRole::UserId).to(User::Table, User::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(string_len(UserRole::RoleId, 24)).foreign_key(ForeignKey::create().name("fk_user_role_role_id").from(UserRole::Table, UserRole::RoleId).to(Role::Table, Role::Id).on_delete(ForeignKeyAction::Cascade).on_update(ForeignKeyAction::Cascade))
                .col(date_time(UserRole::CreatedAt).default("now()"))
                .to_owned()
        ).await?;

        // Default roles and permissions
        let all_permissions = Resource::get_all_permissions()
            .iter()
            .map(|r| (cuid(), r.get_action(), r.get_resource(), format!("{} permission to {}", r.get_resource(), r.get_action())))
            .collect::<Vec<(String, String, String, String)>>();
        
        let all_roles = vec![
            (cuid(), "Admin", "Admin system role"),
            (cuid(), "User", "User system role")
        ];

        // Insert roles
        let mut roles = Query::insert()
            .into_table(Role::Table)
            .columns(vec![Role::Id, Role::Name, Role::Description])
            .to_owned();
        for (id, name, description) in &all_roles {
            roles.values_panic([id.into(), (*name).into(), (*description).into()]);
        }
        manager.exec_stmt(roles).await?;
        
        // Insert permissions
        let mut permissions = Query::insert()
            .into_table(Permission::Table)
            .columns(vec![Permission::Id, Permission::Action, Permission::Resource, Permission::Description]).to_owned();
        for (uid, action, resource, description) in &all_permissions {
            permissions = permissions.values_panic([uid.into(), action.into(), resource.into(), description.into()]).to_owned();
        }
        manager.exec_stmt(permissions.to_owned()).await?;
        
        
        // Insert des role_permission
        let mut role_permissions_insert = Query::insert()
            .into_table(RolePermission::Table)
            .columns(vec![RolePermission::RoleId, RolePermission::PermissionId])
            .to_owned();

        let admin_role_id = &all_roles.iter().find(|(_, a, _)| a.eq(&"Admin")).unwrap().0;
        println!("admin_role_id: {}", admin_role_id);
        
        all_permissions.iter().filter(|(_, a, _, _)| a == "all").map(|(id, _, _, _)| id.clone()).for_each(|x| {
            role_permissions_insert = role_permissions_insert.values_panic([admin_role_id.into(), x.into()]).to_owned();
        });
        manager.exec_stmt(role_permissions_insert.to_owned()).await?;
        
        
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