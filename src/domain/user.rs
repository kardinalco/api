use entity::user::{Model, Entity};
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::domain::house::HouseDomain;
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;

pub struct UserDomain;

impl UserDomain {
    
    pub async fn list_user(session: AuthSession, db:  DatabaseConnection) -> Result<(), Error> {
        let _a = HouseDomain::list_house(session, db).await;
        Ok(())
    } 
    
    pub async fn find_active_user_by_id(user_id: String, db: &DatabaseConnection) -> Result<Model, Error> {
        Entity::find_by_id(user_id)
            .one(db)
            .await
            ?.ok_or(Error::Entity(EntityError::NotFound("".to_string())))
    }
    
}