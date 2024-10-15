use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;
use crate::db::models::user::{InsertUser, UpdateUser, User};
use crate::db::schema::user::dsl::*;
use crate::exceptions::db::DatabaseError;

pub struct UserRepo;

impl UserRepo {
    pub fn find_user_by_id(conn: &mut PgConnection, id_to_find: String) -> Result<User, DatabaseError> {
        Ok(user
            .select(User::as_select())
            .filter(id.eq(Uuid::parse_str(&id_to_find).unwrap()))
            .first::<User>(conn)?)
    }
    
    pub fn find_user_by_email(conn: &mut PgConnection, email_to_find: String) -> Result<User, DatabaseError> {
        Ok(user
            .select(User::as_select())
            .filter(email.eq(email_to_find))
            .first::<User>(conn)?)
    }
    
    pub fn create_user(conn: &mut PgConnection, new_user: InsertUser) -> Result<User, DatabaseError> {
        Ok(diesel::insert_into(user)
            .values(&new_user)
            .get_result::<User>(conn)?)
    }
    
    pub fn update_user(conn: &mut PgConnection, id_to_update: String, updated_user: UpdateUser) -> Result<User, DatabaseError> {
        let result = diesel::update(user)
            .filter(id.eq(Uuid::parse_str(&id_to_update).unwrap()))
            .set(&updated_user)
            .get_result::<User>(conn)?;
        Ok(result)
    }
    
    pub fn update_user_profile_picture(conn: &mut PgConnection, id_to_update: String, picture_url: String) -> Result<User, DatabaseError> {
        let result = diesel::update(user)
            .filter(id.eq(Uuid::parse_str(&id_to_update).unwrap()))
            .set(picture.eq(Some(picture_url)))
            .get_result::<User>(conn)?;
        Ok(result)
    }
    
}