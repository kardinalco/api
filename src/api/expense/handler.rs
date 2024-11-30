use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

pub struct ExpenseRoute;

impl ExpenseRoute {
    pub async fn get_expense(_session: AuthSession) -> Result<String, Error> {
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

    pub async fn list_payment() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn get_payment() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn create_payment() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_payment() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_payment() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn list_refund() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn get_refund() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn create_refund() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn update_refund() -> Result<String, Error> {
        Ok("".to_string())
    }

    pub async fn delete_refund() -> Result<String, Error> {
        Ok("".to_string())
    }
}

impl Route for ExpenseRoute {
    fn route(cfg: &mut ServiceConfig) {
        cfg.service(
            scope("/expense")
                .route("/", get().to(Self::list_expense))
                .route("/{id}", get().to(Self::get_expense))
                .route("/{id}", put().to(Self::update_expense))
                .route("/{id}", delete().to(Self::delete_expense))
                .route("/expense/{id}/payments", get().to(Self::list_payment))
                .route("/expense/{id}/payments", post().to(Self::create_payment))
                .route("/expense/{id}/payments/{paymentId}", get().to(Self::get_payment))
                .route("/expense/{id}/payments/{paymentId}", put().to(Self::update_payment))
                .route("/expense/{id}/payments/{paymentId}", delete().to(Self::delete_payment))
                .route("/expense/{id}/refund/", get().to(Self::list_refund))
                .route("/expense/{id}/refund/", post().to(Self::create_refund))
                .route("/expense/{id}/refund/{refundId}", get().to(Self::get_refund))
                .route("/expense/{id}/refund/{refundId}", put().to(Self::update_refund))
                .route("/expense/{id}/refund/{refundId}", delete().to(Self::delete_refund))
        );
    }
}
