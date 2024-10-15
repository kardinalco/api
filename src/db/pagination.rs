use diesel::{PgConnection, RunQueryDsl};
use diesel::query_dsl::LoadQuery;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

impl<T> PaginatedResult<T> {
    pub fn new(data: Vec<T>, total: i64, limit: i64, offset: i64) -> Self {
        Self {
            data,
            total,
            limit,
            offset,
        }
    }
}