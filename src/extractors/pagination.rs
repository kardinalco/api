use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::{Query};
use serde::Deserialize;
use tracing::instrument;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_PER_PAGE: u64 = 30;

#[derive(Debug)]
pub struct Pagination {
    pub page: u64,
    pub limit: u64,
    pub bypass: bool,
}

#[derive(Debug, Deserialize)]
pub struct PagQuery {
    pub page: Option<u64>,
    pub limit: Option<u64>,
}

impl Pagination {
    pub fn new(page: u64, limit: u64) -> Self {
        Pagination { page, limit, bypass: false }
    }
    
    pub fn all() -> Self {
        Pagination { page: 0, limit: 0, bypass: true}
    }
}

impl Into<Pagination> for PagQuery {
    fn into(self) -> Pagination {
        Pagination {
            page: self.page.unwrap_or(DEFAULT_PAGE),
            limit: self.limit.unwrap_or(DEFAULT_PER_PAGE),
            bypass: false
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination { page: DEFAULT_PAGE, limit: DEFAULT_PER_PAGE, bypass: false }
    }
}

impl FromRequest for Pagination {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    #[instrument(level = "info", name = "pagination::from_request", skip(req))]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            Ok(Query::<PagQuery>::extract(&request)
                .await
                .map(|q| q.into_inner().into())?
            )
        })
    }
}