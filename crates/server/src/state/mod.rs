use std::sync::Arc;
use db::redis::RedisClient;
use crate::errors::AppResult;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<db::Pool>,
    pub redis: Arc<RedisClient>,
}

impl AppState {
    pub async fn new(pool: db::Pool, redis: Arc<RedisClient>) -> AppResult<Self> {
        Ok(Self {
            pool: Arc::new(pool),
            redis,
        })
    }
}
