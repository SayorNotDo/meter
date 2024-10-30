use std::collections::HashMap;

use test_context::AsyncTestContext;

use crate::helper::user::{Role, TestUser};

use super::state::TestContext;

pub struct SeedDbTestContext {
    pub app: TestContext,
    pub users: HashMap<Role, TestUser>,
}

impl AsyncTestContext for SeedDbTestContext {
    async fn setup() -> Self {
        let app = TestContext::setup().await;
        let users = TestUser::create_user(&app.state)
            .await
            .expect("failed to create test users");
        Self { app, users }
    }

    async fn teardown(self) {
        self.app.teardown().await
    }
}
