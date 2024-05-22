use crate::errors::CustomError;
use axum::Json;

use tokio_postgres::error::DbError;

pub trait BaseDao<T> {
    async fn all(&self) -> Result<Json<Vec<T>>, CustomError>;
    async fn insert(&self, object: &T) -> Result<(), CustomError>;
    async fn get_by_id(&self, id: i32) -> Result<T, DbError>;
    async fn update(&self, object: &T) -> Result<T, DbError>;
    async fn delete_by_id(&self, id: i32) -> Result<T, DbError>;
}
