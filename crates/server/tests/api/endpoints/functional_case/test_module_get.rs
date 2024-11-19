use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use server::dto::request::user::LoginRequest;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_case_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let _token = ctx.app.api.get_token(&req).await.unwrap();
}
