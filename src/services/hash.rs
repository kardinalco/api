use crate::exceptions::error::Error;

pub fn hash(value: &str) -> Result<String, Error> {
    Ok(bcrypt::hash(value, 6)?)
}

pub fn compare(hashed_value: &str, value: &str) -> Result<bool, Error> {
    Ok(bcrypt::verify(value, hashed_value)?)
}
