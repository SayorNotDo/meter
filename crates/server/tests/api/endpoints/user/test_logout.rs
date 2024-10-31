use crate::context::seeder::SeedDbTestContext;
use crate::helper::user::Role;
use server::dto::request::LoginRequest;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_logout(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();
    let (status, _resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
    assert!(status.is_success(), "status: {status}");
}
