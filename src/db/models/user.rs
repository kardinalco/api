use diesel::prelude::*;
use diesel_ulid::DieselUlid;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Queryable, Identifiable, Selectable, Serialize, Deserialize, PartialEq, Clone, Associations)]
#[diesel(belongs_to(User, foreign_key = created_by))]
#[diesel(table_name = crate::db::schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub phone_number: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub is_deleted: bool,
    pub is_verified: bool,
    pub picture: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, PartialEq, Clone)]
#[diesel(table_name = crate::db::schema::user)]
pub struct InsertUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub phone_number: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub is_verified: bool,
    pub picture: Option<String>,
    pub created_by: Option<Uuid>,
}

#[derive(AsChangeset, Debug, Serialize, Deserialize, PartialEq, Clone)]
#[diesel(table_name = crate::db::schema::user)]
pub struct UpdateUser {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub birthdate: Option<chrono::NaiveDate>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
    pub is_deleted: Option<bool>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub deleted_by: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::establish_test_connection;

    #[test]
    fn test_create_user() {
        establish_test_connection();
    }
    
    fn test_find_user() {
        
    }
    
    fn test_update_user() {
        
    }
    
    fn test_delete_user() {
        
    }
}