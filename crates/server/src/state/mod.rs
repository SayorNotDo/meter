use std::sync::Arc;
use db::redis::RedisClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: db::Pool,
    pub redis: Arc<RedisClient>,
}

impl AppState {
    pub fn new(pool: db::Pool, redis: Arc<RedisClient>) -> Self {
        Self {
            pool,
            redis,
        }
    }
}
