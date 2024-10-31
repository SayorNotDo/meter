use crate::{
    configure::Config,
    errors::AppResult,
    utils::{http::HttpClient, smtp::{EmailClient, email_client_builder}, ClientBuilder},
};
use db::{redis::RedisClient, create_pool, redis_client_builder};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub pool: Arc<db::Pool>,
    pub redis: Arc<RedisClient>,
    pub email: Arc<EmailClient>,
    pub http: HttpClient,
}

impl AppState {
    pub async fn new(config: Config) -> AppResult<Self> {
        let pool = Arc::new(create_pool(&config.storage.database_url));
        let redis = Arc::new(redis_client_builder(&config.storage.redis_url));
        let email = Arc::new(email_client_builder(&config.smtp));
        let http = HttpClient::build_from_config(&config)?;
        Ok(Self {
            config: Arc::new(config),
            pool,
            redis,
            email,
            http,
        })
    }
}
