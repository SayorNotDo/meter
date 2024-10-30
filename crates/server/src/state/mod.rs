use crate::{
    configure::Config,
    errors::AppResult,
    utils::{http::HttpClient, smtp::EmailClient},
};
use db::redis::RedisClient;
use std::sync::Arc;
use db::create_pool;
use crate::utils::{smtp, ClientBuilder};

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<db::Pool>,
    pub redis: Arc<RedisClient>,
    pub email: Arc<EmailClient>,
    pub http: HttpClient,
}

impl AppState {
    pub async fn new(config: Config) -> AppResult<Self> {
        let pool = create_pool(&config.storage.database_url);
        let redis = db::redis_client_builder(&config.storage.redis_url);
        let email = smtp::email_client_builder(&config.smtp);
        let http = HttpClient::build_from_config(&config)?;
        Ok(Self {
            pool: Arc::new(pool),
            redis: Arc::new(redis),
            email: Arc::new(email),
            http,
        })
    }
}
