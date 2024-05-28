use tokio_postgres::error::DbError;

use crate::errors::AppResult;

pub trait BaseDao<T> {
    async fn all(&self) -> AppResult<Vec<T>>;
    async fn insert(&self, object: &T) -> AppResult<i32>;
    async fn get_by_id(&self, id: i32) -> Result<T, DbError>;
    async fn update(&self, object: &T) -> Result<T, DbError>;
    async fn delete_by_id(&self, id: i32) -> Result<T, DbError>;
}
