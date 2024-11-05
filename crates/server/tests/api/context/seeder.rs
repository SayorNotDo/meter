use std::collections::HashMap;

use crate::helper::{
    project::TestProject,
    user::{Role, TestUser},
};
use test_context::AsyncTestContext;

use super::state::TestContext;

pub struct SeedDbTestContext {
    pub app: TestContext,
    pub users: HashMap<Role, TestUser>,
    pub project: TestProject,
}

impl AsyncTestContext for SeedDbTestContext {
    async fn setup() -> Self {
        let app = TestContext::setup().await;
        let users = TestUser::create_user(&app.state.pool)
            .await
            .expect("Failed to create test users...");
        let project = TestProject::get_default_project(
            &app.state.pool,
            users.get(&Role::System).unwrap().uuid,
        )
        .await
        .expect("Failed to create test project");
        Self {
            app,
            users,
            project,
        }
    }

    async fn teardown(self) {
        for user in self.users.iter() {
            TestUser::enable_user(&self.app.state.pool, user.1.id)
                .await
                .expect("Failed to enable user");
        }
        self.app.teardown().await
    }
}
