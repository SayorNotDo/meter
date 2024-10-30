use crate::context::seeder::SeedDbTestContext;
use test_context::test_context;
use server::dto::request::LoginRequest;
use crate::helper::user::Role;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_login(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
}
