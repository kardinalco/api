use actix_web::web::{get, post, ServiceConfig};
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::utils::route::Route;

pub struct ExpenseHandler;

impl ExpenseHandler {
    pub async fn get_expense(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }
    
    pub async fn create_expense(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn list_expense(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_expense(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_expense(_session: AuthSession) -> Result<String, Error> {
        Ok("".to_string())
    }
    
}

impl Route for ExpenseHandler {
    fn route(cfg: &mut ServiceConfig) {
        cfg.route("/expense", get().to(Self::list_expense))
            .route("/expense", post().to(Self::create_expense))
            .route("/expense/{id}", get().to(Self::get_expense))
            .route("/expense/{id}", actix_web::web::put().to(Self::update_expense))
            .route("/expense/{id}", actix_web::web::delete().to(Self::delete_expense));
    }
}