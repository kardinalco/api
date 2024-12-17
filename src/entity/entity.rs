use sea_orm::ConnectionTrait;

pub trait Create {
    type QueryResult;
    type Body;
    async fn create(db: &impl ConnectionTrait, body: Self::Body, created_by: Option<String>) -> Self::QueryResult;
}

pub trait Update {
    type QueryResult;
    type Body;
    async fn update(self, db: &impl ConnectionTrait, body: Self::Body, updated_by: Option<String>) -> Self::QueryResult;
}

pub trait SoftDelete {
    type QueryResult;
    async fn soft_delete(self, db: &impl ConnectionTrait, deleted_by: Option<String>) -> Self::QueryResult;
}