use tracing::instrument;
use crate::exceptions::error::Error;

#[instrument(name = "hash::hash", skip(value))]
pub fn hash(value: &str) -> Result<String, Error> {
    Ok(bcrypt::hash(value, 6)?)
}

#[instrument(name = "hash::compare", skip(hashed_value, value))]
pub fn compare(value: &str, hashed_value: &str) -> Result<bool, Error> {
    Ok(bcrypt::verify(value, hashed_value)?)
}
