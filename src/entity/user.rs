use cuid2::cuid;
use entity::user::Column;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use crate::api::auth::request::AuthRegisterRequest;
use crate::exceptions::error::Error;
use crate::api::user::request::UserUpdateRequest;
use crate::extractors::filter::IntoColumn;
use crate::services::hash::hash;

#[derive(Debug, Clone, Deserialize)]
pub enum UserFields {
    Id,
    Firstname,
    Lastname,
    Email,
    IsDeleted,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}

impl IntoColumn for UserFields {
    fn into_col(self) -> Column {
        match self {
            UserFields::Id => Column::Id,
            UserFields::Firstname => Column::FirstName,
            UserFields::Lastname => Column::LastName,
            UserFields::Email => Column::Email,
            UserFields::IsDeleted => Column::IsDeleted,
            UserFields::CreatedAt => Column::CreatedAt,
            UserFields::CreatedBy => Column::CreatedBy,
            UserFields::UpdatedAt => Column::UpdatedAt,
            UserFields::UpdatedBy => Column::UpdatedBy,
        }
    }
}

type Model = entity::user::Model;
type Entity = entity::user::Entity;

pub trait CreateUser: Sized {
    type QueryResult;
    async fn create(db: &DatabaseConnection, body: AuthRegisterRequest, created_by: Option<String>) -> Self::QueryResult;
}

impl CreateUser for Entity {
    type QueryResult = Result<Model, Error>;

    /// Create a new user with a hashed password and return the user model
    async fn create(db: &DatabaseConnection, body: AuthRegisterRequest, created_by: Option<String>) -> Self::QueryResult {
        let model = entity::user::ActiveModel {
            id: Set(cuid()),
            email: Set(body.email),
            first_name: Set(body.firstname),
            last_name: Set(body.lastname),
            password: Set(hash(&body.password)?),
            ..Default::default()
        };
        let mut model = model.into_active_model();
        model.created_by = Set(created_by);
        Ok(model.insert(db).await?)
    }
}

pub trait UpdateUser: Sized {
    type QueryResult;
    async fn update(self, db: &DatabaseConnection, body: UserUpdateRequest, updated_by: Option<String>) -> Self::QueryResult;
    async fn update_password(self, db: &DatabaseConnection, password: &str, updated_by: Option<String>) -> Self::QueryResult;
    async fn update_mail(self, db: &DatabaseConnection, mail_id: i32, updated_by: Option<String>) -> Self::QueryResult;
    async fn update_profile_picture(self, db: &DatabaseConnection, profile_picture: &str, updated_by: Option<String>) -> Self::QueryResult;
}

impl UpdateUser for entity::user::Model {
    type QueryResult = Result<Self, Error>;

    async fn update(self, db: &DatabaseConnection, body: UserUpdateRequest, updated_by: Option<String>) -> Self::QueryResult {
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

    async fn update_password(self, db: &DatabaseConnection, password: &str, updated_by: Option<String>) -> Self::QueryResult {
        let mut a = self.into_active_model();
        a.password = Set(password.to_string());
        a.updated_by = Set(updated_by);
        a.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(a.update(db).await?)
    }

    async fn update_mail(self, db: &DatabaseConnection, mail_id: i32, updated_by: Option<String>) -> Self::QueryResult {
        let mut a = self.into_active_model();
        a.email_id = Set(Some(mail_id));
        a.updated_by = Set(updated_by);
        a.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
        Ok(a.update(db).await?)
    }

    async fn update_profile_picture(self, db: &DatabaseConnection, profile_picture: &str, updated_by: Option<String>) -> Self::QueryResult {
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