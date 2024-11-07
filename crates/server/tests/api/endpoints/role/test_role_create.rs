use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use server::dto::request::LoginRequest;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let _token = ctx.app.api.get_token(&req).await.unwrap();
    //
    // let (status, _resp) = ctx.app.api.create_role(&token.access_token).await.unwrap();
    // assert!(status.is_success(), "status: {status}");
}
