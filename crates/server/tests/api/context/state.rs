use db::create_pool;
use once_cell::sync::Lazy;
use server::constant::ENV_PREFIX;
use server::state::AppState;
use server::{
    configure::{env::get_env_source, Config},
    utils,
};
use std::sync::Arc;
use test_context::AsyncTestContext;

use crate::helper::{api::Api, INIT_SUBCRIBER};

pub struct TestContext {
    pub state: AppState,
    pub api: Api,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        Lazy::force(&INIT_SUBCRIBER);
        let config = Config::read(get_env_source(ENV_PREFIX)).unwrap();

        let pool = create_pool(&config.storage.database_url);

        let redis = Arc::new(db::redis_client_builder(&config.storage.redis_url));
        let email = Arc::new(utils::smtp::email_client_builder(&config.smtp));
        let api = Api::new(&config.http);
        let state = server::state::AppState::new(pool, redis, email)
            .await
            .unwrap();
        Self { state, api }
    }

    async fn teardown(self) -> () {
        /* TODO: test-app shutdown code */
    }
}
