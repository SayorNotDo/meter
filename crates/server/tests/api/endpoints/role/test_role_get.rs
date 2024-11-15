use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{
        result::AppResponseResult,
        user::{Role, TestUser},
    },
};
use server::{dao::entity::UserRole, dto::request::user::LoginRequest, errors::AppResponseError};
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

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_failure_get_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_user_role(&token.access_token, ctx.project.id, 0)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| {
        e.kind == "ROLE_NOT_FOUND_ERROR"
    })
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_access_denied_get_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let role = ctx.users.get(&Role::User).unwrap();
    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();
    let (status, resp) = ctx
        .app
        .api
        .get_user_role(&token.access_token, ctx.project.id, role.role_id)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| { e.kind == "FORBIDDEN_ERROR" })
}
