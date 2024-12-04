use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, RelationTrait};
use sea_orm::JoinType::LeftJoin;
use tracing::instrument;
use crate::api::expense::request::ExpenseCreateRequest;
use crate::exceptions::entity::EntityError;
use crate::exceptions::error::Error;
use crate::extractors::auth_session::AuthSession;

pub struct ExpenseDomain;

type Expense = entity::expense::Model;

impl ExpenseDomain {

    #[instrument(skip(db))]
    pub async fn get_expense(db: &DatabaseConnection, expense_id: &str) -> Result<Expense, Error> {
        Ok(entity::expense::Entity::find()
            .filter(entity::expense::Column::Id.eq(expense_id))
            .one(db)
            .await?
            .ok_or(Error::Entity(EntityError::NotFound("Expense", expense_id.to_string())))?)
    }

    #[instrument(skip(db))]
    pub async fn list_expense(db: &DatabaseConnection, house_id: &str) -> Result<Vec<Expense>, Error> {
        Ok(entity::expense::Entity::find()
            .join(LeftJoin, entity::expense::Relation::ExpenseHouse.def())
            .filter(entity::expense_house::Column::HouseId.eq(house_id))
            .all(db)
            .await?)
    }

    #[instrument(skip(db, session))]
    pub async fn create_expense(session: AuthSession, db: &DatabaseConnection, body: ExpenseCreateRequest) -> Result<Expense, Error> {
        Ok(body.into_model(&session.user.id).insert(db).await?)
    }
    
}