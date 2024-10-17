use bcrypt::{DEFAULT_COST, hash, verify};

use crate::exceptions::error::Error;

pub struct HashValue<'a>(&'a str);

impl<'a> HashValue<'a> {
    pub fn hash(&self) -> Result<String, Error> {
        Ok(hash(self.0, DEFAULT_COST)?)
    }

    pub fn compare(hashed_value: &str, value: &str) -> Result<bool, Error> {
        Ok(verify(value, hashed_value)?)
    }
}
