use std::future::Future;
use std::pin::Pin;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::web::{Json};
use entity::user::Column;
use sea_query::Expr;
use sea_query::extension::postgres::PgExpr;
use serde::{Deserialize};
use serde::de::DeserializeOwned;
use tracing::instrument;
use crate::exceptions::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub enum Operator {
    Equals,
    EqualsIgnoreCase,
    Contains,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterOrEquals,
    LessOrEquals,
    Like,
}

#[derive(Debug, Clone, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Condition<T: IntoColumn> {
    pub field: T,
    pub operator: Operator,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CompositeCondition<T: IntoColumn> {
    pub logical_operator: LogicalOperator,
    pub sub_conditions: Vec<Condition<T>>,
    pub sub_composites: Vec<CompositeCondition<T>>,
}

pub trait IntoColumn: Clone {
    fn into_col(self) -> Column;
}

pub fn build_condition<T: IntoColumn>(condition: &CompositeCondition<T>) -> sea_orm::Condition {
    let mut expr = match condition.clone().logical_operator {
        LogicalOperator::And => sea_orm::Condition::all(),
        LogicalOperator::Or => sea_orm::Condition::any(),
    };
    for sub_condition in condition.clone().sub_conditions {
        let column = Expr::col(sub_condition.clone().field.into_col());
        let value = Expr::val(&sub_condition.value);
        let sub_expr = match sub_condition.operator {
            Operator::Equals => column.eq(value),
            Operator::NotEquals => column.ne(value),
            Operator::GreaterThan => column.gt(value),
            Operator::LessThan => column.lt(value),
            Operator::GreaterOrEquals => column.gte(value),
            Operator::LessOrEquals => column.lte(value),
            Operator::Contains => column.contains(value),
            Operator::EqualsIgnoreCase => column.eq(value),
            Operator::Like => column.like("TODO: changed this"),
        };
        expr = expr.add(sub_expr);
    }
    for sub_composite in &condition.sub_composites {
        expr = expr.add(build_condition(sub_composite));
    }
    expr
}

#[derive(Debug, Deserialize)]
pub struct Filter<T: IntoColumn>(pub Option<CompositeCondition<T>>);

impl<T: IntoColumn + DeserializeOwned + 'static> FromRequest for Filter<T> {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    #[instrument(level = "info", name = "filter::from_request", skip(req, payload))]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let json_extract = Json::<Option<CompositeCondition<T>>>::from_request(req, payload);
        Box::pin(async move {
            let value = json_extract
                .await
                .map_err(|x| Error::Parse(x.to_string()))?
                .into_inner();
            Ok(Filter::<T>(value))
        })
    }
}

impl<T: IntoColumn> Into<Filter<T>> for Option<CompositeCondition<T>> {
    fn into(self) -> Filter<T> {
        Filter(self)
    }

}

impl<T: IntoColumn> Filter<T> {
    pub fn into_inner(self) -> Option<CompositeCondition<T>> {
        self.0
    }
}