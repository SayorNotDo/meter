use crate::context::seeder::SeedDbTestContext;
use crate::helper::user::Role;
use server::dto::request::LoginRequest;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_delete_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let role = ctx.users.get(&Role::User).unwrap();

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let (status, resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, role.role_id)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");
}
