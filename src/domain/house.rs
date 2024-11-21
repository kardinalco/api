use entity::house::Model;
use entity::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbBackend, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect, RelationTrait, Statement};
use sea_orm::ActiveValue::Set;
use sea_orm::JoinType::LeftJoin;
use sea_orm::sea_query::Alias;
use crate::api::house::request::{HouseCreateRequest, HouseInviteUserRequest, HouseRevokeUserRequest, HouseUpdateRequest};
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;

pub struct HouseDomain;

// 0801820036

impl HouseDomain {
    
    pub async fn find_active_house_by_id(house_id: &String, db: &DatabaseConnection) -> Result<Model, Error> {
        House::find_by_id(house_id)
            .filter(entity::house::Column::DeletedAt.is_null())
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("House".to_string())))
    }
    
    pub async fn get_active_house_with_users(house_id: &String, db: &DatabaseConnection) -> Result<(Model, Vec<entity::user::Model>), Error> {
        let house_with_users = House::find_by_id(house_id.clone())
            .filter(entity::house::Column::DeletedAt.is_null())
            .join_as(LeftJoin, entity::house::Relation::User3.def(), Alias::new("created_by_user"))
            .join_as(LeftJoin, entity::house::Relation::User2.def(), Alias::new("updated_by_user"))
            .join_as(LeftJoin, entity::house::Relation::User1.def(), Alias::new("deleted_by_user"))
            .find_with_related(entity::user::Entity)
            .all(db)
            .await?;
        let (house, users) = match house_with_users.get(0) {
            Some(house) => Ok(house.to_owned()),
            None => Err(Error::Entity(EntityError::NotFound("House".to_string())))
        }?;
        Ok((house, users))
    }
    
    pub async fn create_house(session: AuthSession, body: HouseCreateRequest, db: DatabaseConnection) -> Result<Model, Error> {
        let mut house = body.into_model();
        house.created_by = Set(session.user.id);
        Ok(house.insert(&db).await?)
    }
    
    pub async fn list_house(session: AuthSession, db: DatabaseConnection) -> Result<Vec<Model>, Error> {
        let a = User::find()
            .from_raw_sql(Statement::from_sql_and_values(DbBackend::Postgres, r#"SELECT u.* FROM "user" u INNER JOIN "house_user" hu ON u.id = hu.user_id WHERE hu.house_id IN (SELECT house_id FROM "house_user" GROUP BY house_id HAVING COUNT(user_id) > 1);"#, []))
            .all(&db).await?;

        println!("{:?}", a);

        Ok(House::find()
                .filter(entity::house::Column::CreatedBy.eq(session.user.id))
                .filter(entity::house::Column::DeletedAt.is_null())
                .all(&db)
                .await
                .unwrap_or(vec![]))
    }
    
    pub async fn invite_users(_: AuthSession, _: HouseInviteUserRequest, _: DatabaseConnection) -> Result<(), Error> {
        todo!()
        
    }
    
    pub async fn revoke_users(_: AuthSession, _: HouseRevokeUserRequest, _: DatabaseConnection) -> Result<(), Error> {
        todo!()
    }

    pub async fn update_house(session: AuthSession, db: DatabaseConnection, house_id: String, body: HouseUpdateRequest) -> Result<Model, Error> {
        let house_with_users = Self::get_active_house_with_users(&house_id, &db).await?;
        Self::need_to_be_members(&house_with_users, &session)?;
        let mut model = house_with_users.0.into_active_model();
        model.address = body.address.map_or(model.address, |address| Set(Some(address)));
        model.name = body.name.map_or(model.name, |name| Set(name));
        model.description = body.description.map_or(model.description, |description| Set(Some(description)));
        model.location_type = body.location_type.map_or(model.location_type, |location_type| Set(location_type.into()));
        model.owner_name = body.owner_name.map_or(model.owner_name, |owner_name| Set(Some(owner_name)));
        model.owner_contact_info = body.owner_contact_info.map_or(model.owner_contact_info, |owner_contact_info| Set(Some(owner_contact_info)));
        model.owner_email = body.owner_email.map_or(model.owner_email, |owner_email| Set(Some(owner_email)));
        model.owner_phone = body.owner_phone.map_or(model.owner_phone, |owner_phone| Set(Some(owner_phone)));
        model.built_year = body.built_year.map_or(model.built_year, |built_year| Set(Some(built_year)));
        model.acquired_at = body.acquired_at.map_or(model.acquired_at, |acquired_at| Set(Some(acquired_at)));
        model.updated_by = Set(Some(session.user.id));
        model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(model.update(&db).await?)
    }
    
    pub async fn delete_house(session: AuthSession, db: DatabaseConnection, house_id: String) -> Result<(), Error> {
        let house = Self::find_active_house_by_id(&house_id, &db).await?;
        Self::need_to_be_owner(&house, &session)?;
        let mut active_house = house.into_active_model();
        active_house.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
        active_house.deleted_by = Set(Some(session.user.id));
        active_house.update(&db).await?;
        Ok(())
    }
    
    pub fn need_to_be_owner(house: &Model, session: &AuthSession) -> Result<(), Error> {
        if house.created_by != session.user.id {
            return Err(Error::Entity(EntityError::NeedToBeOwner("House".to_string())))
        }
        Ok(())
    }

    pub fn need_to_be_members(house_with_user: &(Model, Vec<entity::user::Model>), session: &AuthSession) -> Result<(), Error> {
        let (_, users) = house_with_user;
        let user_ids: Vec<String> = users.iter().map(|user| user.id.clone()).collect();
        if !user_ids.contains(&session.user.id) && house_with_user.0.created_by != session.user.id {
            return Err(Error::Entity(EntityError::NoPermission("House".to_string())))
        }
        Ok(())
    }
}