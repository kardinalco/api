use diesel::result::Error;
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
}

impl From<Error> for DatabaseError {
    fn from(e: Error) -> Self {
        match e {
            Error::NotFound => DatabaseError::QueryError,
            Error::DatabaseError(_, _) => DatabaseError::QueryError,
            _ => DatabaseError::QueryError,
        }
    }
}

impl From<r2d2::Error> for DatabaseError {
    fn from(value: r2d2::Error) -> Self {
        todo!()
    }
}