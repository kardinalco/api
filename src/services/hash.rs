use tracing::instrument;
use crate::exceptions::error::Error;

#[instrument(skip(value))]
pub fn hash(value: &str) -> Result<String, Error> {
    Ok(bcrypt::hash(value, 6)?)
}

#[instrument(skip(hashed_value, value))]
pub fn compare(hashed_value: &str, value: &str) -> Result<bool, Error> {
    Ok(bcrypt::verify(value, hashed_value)?)
}
