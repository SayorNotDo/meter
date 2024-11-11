use crate::context::seeder::SeedDbTestContext;
use crate::helper::{result::AppResponseResult, user::Role};
use server::{dao::entity::UserRole, dto::request::LoginRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let role = ctx.users.get(&Role::User).unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_user_role(&token.access_token, ctx.project.id, role.role_id)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");
    assert!(matches!(resp, AppResponseResult::Ok(UserRole { .. })))
}
