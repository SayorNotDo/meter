use tokio::task::JoinHandle;

use once_cell::sync::Lazy;
use server::configure::{env::get_env_source, Config};
use server::constant::ENV_PREFIX;
use server::errors::AppResult;
use server::state::AppState;
use test_context::AsyncTestContext;
use tracing::info;

use crate::helper::{api::Api, INIT_SUBCRIBER};
use wiremock::MockServer;

pub struct TestContext {
    pub tasks: Vec<JoinHandle<AppResult>>,
    pub state: AppState,
    pub api: Api,
    #[allow(dead_code)]
    pub mock_server: MockServer,
}

impl AsyncTestContext for TestContext {
    async fn setup() -> Self {
        Lazy::force(&INIT_SUBCRIBER);
        let config = Config::read(get_env_source(ENV_PREFIX)).unwrap();
        let server = server::server::AppServer::new(config.clone())
            .await
            .unwrap();
        let state = server.state.clone();
        let server_task = tokio::task::spawn(server.run());
        let mock_server = MockServer::start().await;
        let api = Api::new(&config.http);
        let tasks = vec![server_task];
        Self {
            tasks,
            state,
            api,
            mock_server,
        }
    }

    async fn teardown(self) -> () {
        /* TODO: test-app shutdown code */
        for task in self.tasks {
            task.abort();
        }
        info!("Teardown done successfully...")
    }
}
