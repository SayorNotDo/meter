use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{
        result::AppResponseResult,
        user::{Role, TestUser},
    },
};
use server::{
    dto::{request::user::LoginRequest, response::ListUserResponse},
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_user_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");

    assert!(matches!(
        resp,
        AppResponseResult::Ok(ListUserResponse { .. })
    ))
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_no_permission_user_get_list(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();

    let req: LoginRequest = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_user_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
    assert_err!(resp, |e: &AppResponseError| e.kind == "FORBIDDEN_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_disabled_user_get_list(ctx: &mut SeedDbTestContext) {
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
        .get_user_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
    assert_err!(resp, |e: &AppResponseError| e.kind == "FORBIDDEN_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_deleted_user_get_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    /* Disable&Delete User */
    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();
    TestUser::delete_user(&ctx.app.state, vec![admin.id])
        .await
        .unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_user_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::UNAUTHORIZED);
    assert_err!(resp, |e: &AppResponseError| e.kind == "UNAUTHORIZED_ERROR");
}
