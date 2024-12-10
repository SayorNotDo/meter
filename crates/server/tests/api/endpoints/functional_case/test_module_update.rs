use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use fake::{Fake, Faker};
use server::dto::request::{file::UpdateModuleRequest, user::LoginRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
pub async fn test_success_update_case_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: UpdateModuleRequest = UpdateModuleRequest {
        id: 0,
        name: Faker.fake::<String>(),
        parent_id: None,
    };

    let (status, resp) = ctx
        .app
        .api
        .update_case_module(&token.access_token, project_id, &req)
        .await
        .unwrap();

    assert!(status.is_success());
}
