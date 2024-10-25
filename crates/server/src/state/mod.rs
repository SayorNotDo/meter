use crate::{
    errors::AppResult,
    utils::{http::HttpClient, smtp::EmailClient},
};
use db::redis::RedisClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<db::Pool>,
    pub redis: Arc<RedisClient>,
    pub email: Arc<EmailClient>,
    pub http: HttpClient,
}

impl AppState {
    pub async fn new(
        pool: db::Pool,
        redis: Arc<RedisClient>,
        email: Arc<EmailClient>,
        http: HttpClient,
    ) -> AppResult<Self> {
        Ok(Self {
            pool: Arc::new(pool),
            redis,
            email,
            http,
        })
    }
}
