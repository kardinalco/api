use crate::api::user::request::UserUpdateRequest;
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use entity::user::{Entity, Model};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set};

pub struct UserDomain;

impl UserDomain {

    pub async fn find_active_user_by_id(
        user_id: &String,
        db: &DatabaseConnection,
    ) -> Result<Model, Error> {
        Entity::find_by_id(user_id)
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound(
                "User",
                user_id.to_owned(),
            )))
    }

    pub async fn update(updated_by: String, user_id: String, db: &DatabaseConnection, body: UserUpdateRequest) -> Result<Model, Error> {
        let user = Entity::find_by_id(user_id.clone())
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("User", user_id)))?;
        let mut model = user.into_active_model();
        model.address = body.address.map_or(model.address, |address| Set(Some(address)));
        model.first_name = body.firstname.map_or(model.first_name, |first_name| Set(first_name));
        model.last_name = body.lastname.map_or(model.last_name, |last_name| Set(last_name));
        model.phone_number = body.phone_number.map_or(model.phone_number, |phone| Set(Some(phone)));
        model.birthday = body.birthday.map_or(model.birthday, |birthday| Set(Some(birthday)));
        model.city = body.city.map_or(model.city, |city| Set(Some(city)));
        model.country = body.country.map_or(model.country, |country| Set(Some(country)));
        model.zip_code = body.zip_code.map_or(model.zip_code, |zip_code| Set(Some(zip_code)));
        model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        model.updated_by = Set(Some(updated_by));
        Ok(model.update(db).await?)
    }
}
