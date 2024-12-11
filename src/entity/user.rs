use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use crate::exceptions::error::Error;
use crate::api::user::request::UserUpdateRequest;

pub trait UpdateUser: Sized {
    async fn update(self, db: &DatabaseConnection, body: UserUpdateRequest, updated_by: Option<String>) -> Result<Self, Error>;
    async fn update_password(self, db: &DatabaseConnection, password: &str, updated_by: Option<String>) -> Result<Self, Error>;
    async fn update_mail(self, db: &DatabaseConnection, mail_id: i32, updated_by: Option<String>) -> Result<Self, Error>;
    async fn update_profile_picture(self, db: &DatabaseConnection, profile_picture: &str, updated_by: Option<String>) -> Result<Self, Error>;
}

impl UpdateUser for entity::user::Model {
    async fn update(self, db: &DatabaseConnection, body: UserUpdateRequest, updated_by: Option<String>) -> Result<Self, Error> {
        let mut model = self.into_active_model();
        model.address = body.address.map_or(model.address, |address| sea_orm::Set(Some(address)));
        model.first_name = body.firstname.map_or(model.first_name, |first_name| sea_orm::Set(first_name));
        model.last_name = body.lastname.map_or(model.last_name, |last_name| sea_orm::Set(last_name));
        model.phone_number = body.phone_number.map_or(model.phone_number, |phone| sea_orm::Set(Some(phone)));
        model.birthday = body.birthday.map_or(model.birthday, |birthday| sea_orm::Set(Some(birthday)));
        model.city = body.city.map_or(model.city, |city| sea_orm::Set(Some(city)));
        model.country = body.country.map_or(model.country, |country| sea_orm::Set(Some(country)));
        model.zip_code = body.zip_code.map_or(model.zip_code, |zip_code| sea_orm::Set(Some(zip_code)));
        model.updated_at = sea_orm::Set(Some(chrono::Utc::now().naive_utc()));
        model.updated_by = sea_orm::Set(updated_by);
        Ok(model.update(db).await?)
    }

    async fn update_password(self, db: &DatabaseConnection, password: &str, updated_by: Option<String>) -> Result<Self, Error> {
        let mut a = self.into_active_model();
        a.password = Set(password.to_string());
        a.updated_by = Set(updated_by);
        a.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(a.update(db).await?)
    }

    async fn update_mail(self, db: &DatabaseConnection, mail_id: i32, updated_by: Option<String>) -> Result<Self, Error> {
        let mut a = self.into_active_model();
        a.email_id = Set(Some(mail_id));
        a.updated_by = Set(updated_by);
        a.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(a.update(db).await?)
    }

    async fn update_profile_picture(self, db: &DatabaseConnection, profile_picture: &str, updated_by: Option<String>) -> Result<Self, Error> {
        let mut a = self.into_active_model();
        a.picture = Set(Some(profile_picture.to_string()));
        a.updated_by = Set(updated_by);
        a.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(a.update(db).await?)
    }
}

pub trait DeleteUser {
    async fn delete_user(self, db: &DatabaseConnection, deleted_by: Option<String>) -> Result<Self, Error> where Self: Sized;
}

impl DeleteUser for entity::user::Model {
    async fn delete_user(self, db: &DatabaseConnection, deleted_by: Option<String>) -> Result<Self, Error> where Self: Sized {
        let mut a = self.into_active_model();
        a.deleted_by = Set(deleted_by);
        a.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
        a.is_deleted = Set(true);
        Ok(a.update(db).await?)
    }
}