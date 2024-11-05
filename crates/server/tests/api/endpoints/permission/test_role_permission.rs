use crate::{context::seeder::SeedDbTestContext, helper::user::Role, unwrap};

use crate::helper::user::TestUser;
use server::dto::request::LoginRequest;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_role_permission_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();
    let _resp = unwrap!(resp);
    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_access_denied_get_role_permission_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();
    let _resp = unwrap!(resp);
    assert!(status.is_client_error(), "status: {status}");
}
