use std::sync::Arc;

use crate::helper::api::Api;
use db::create_pool;

use server::state::AppState;
use server::{configure, utils};
use test_context::AsyncTestContext;

pub struct TestContext {
    pub state: AppState,
    pub api: Api,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        let config = configure::Config::parse("./config.toml").unwrap();

        let pool = create_pool(&config.storage.database_url);

        let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
        let email = Arc::new(utils::smtp::email_client_builder(&config.smtp));
        let api = Api::new(&config.http);
        let state = server::state::AppState::new(pool, redis, email)
            .await
            .unwrap();
        Self { state, api }
    }

    async fn teardown(self) -> () {}
}
