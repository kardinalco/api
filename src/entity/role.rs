use cuid2::cuid;
use entity::role::{Entity, Model};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel};
use crate::entity::entity::{Create, SoftDelete, Update};
use crate::exceptions::error::Error;

impl Create for Entity {
    type QueryResult = Result<Model, Error>;
    type Body = crate::api::role::request::CreateRoleBody;

    async fn create(db: &impl ConnectionTrait, body: Self::Body, created_by: Option<String>) -> Self::QueryResult {
        let model = entity::role::ActiveModel {
            id: Set(cuid()),
            name: Set(body.name),
            description: Set(body.description),
            created_by: Set(created_by),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };
        Ok(Entity::insert(model).exec_with_returning(db).await?)
    }
}

impl SoftDelete for Model {
    type QueryResult = Result<Model, Error>;

    async fn soft_delete(self, db: &impl ConnectionTrait, deleted_by: Option<String>) -> Self::QueryResult {
        let mut model = self.into_active_model();
        model.deleted_by = Set(deleted_by);
        model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(model.update(db).await?)
    }
}

impl Update for Model {
    type QueryResult = Result<Model, Error>;
    type Body = crate::api::role::request::UpdateRole;

    async fn update(self, db: &impl ConnectionTrait, body: Self::Body, updated_by: Option<String>) -> Self::QueryResult {
        let mut model = self.into_active_model();
        model.name = body.name.map_or(model.name, |name| sea_orm::Set(name));
        model.description = body.description.map_or(model.description, |description| sea_orm::Set(Some(description)));
        model.updated_by = Set(updated_by);
        model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(model.update(db).await?)
    }
}