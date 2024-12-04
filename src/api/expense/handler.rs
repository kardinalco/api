use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;
use crate::utils::route::Route;
use actix_web::web::{delete, get, post, put, scope, Path, ServiceConfig};
use tracing::instrument;
use permission::expense::ExpensePermission;
use permission::resource::Resource;
use crate::api::expense::request::{ExpenseCreateRequest, ExpenseUpdateRequest};
use crate::domain::expense::ExpenseDomain;
use crate::extractors::db::DbReq;
use crate::extractors::dto::Dto;

pub struct ExpenseRoute;

impl ExpenseRoute {
    #[instrument(skip(session, db))]
    pub async fn get_expense(session: AuthSession, db: DbReq, expense_id: Path<String>) -> Result<String, Error> {
        session.enforce_or(vec![Resource::Expense(ExpensePermission::ReadSelf), Resource::Expense(ExpensePermission::Read)]).await?;
        ExpenseDomain::get_expense(&db.into_inner(), &expense_id).await?;
        Ok("".to_string())
    }

    #[instrument(skip(session, db))]
    pub async fn create_expense(session: AuthSession, db: DbReq, body: Dto<ExpenseCreateRequest>) -> Result<String, Error> {
        session.enforce(Resource::Expense(ExpensePermission::Create)).await?;
        ExpenseDomain::create_expense(session, &db.into_inner(), body.into_inner()).await?;
        Ok("".to_string())
    }

    #[instrument(skip(session, db))]
    pub async fn list_expense(session: AuthSession, db: DbReq, house_id: Path<String>) -> Result<String, Error> {
        session.enforce(Resource::Expense(ExpensePermission::List)).await?;
        Ok("".to_string())
    }

    #[instrument(skip(session, db))]
    pub async fn delete_expense(session: AuthSession, db: DbReq, house_id: Path<String>) -> Result<String, Error> {
        session.enforce_or(vec![Resource::Expense(ExpensePermission::DeleteSelf), Resource::Expense(ExpensePermission::Delete)]).await?;
        Ok("".to_string())
    }

    #[instrument(skip(session, db))]
    pub async fn update_expense(session: AuthSession, db: DbReq, house_id: Path<String>, body: Dto<ExpenseUpdateRequest>) -> Result<String, Error> {
        session.enforce_or(vec![Resource::Expense(ExpensePermission::UpdateSelf), Resource::Expense(ExpensePermission::Update)]).await?;
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
                .route("/", post().to(Self::create_expense))
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
