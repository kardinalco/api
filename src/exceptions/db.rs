use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[error("...")]
pub enum DatabaseError {
    #[error("Error querying the database")]
    QueryError,

    #[error("Error inserting into the database")]
    InsertError,

    #[error("Error updating the database")]
    UpdateError,

    #[error("Error deleting from the database")]
    DeleteError,

    #[error("Connection error: {0}")]
    ConnectionError(String)
}

impl From<sea_orm::error::DbErr> for DatabaseError {
    fn from(value: sea_orm::error::DbErr) -> Self {
        match value {
            _ => Self::ConnectionError(String::from("TODO"))
        }
    }
}