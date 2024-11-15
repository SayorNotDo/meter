use crate::{
    context::seeder::SeedDbTestContext,
    helper::user::{Role, TestUser},
};
use server::dto::request::user::{LoginRequest, UpdateUserStatusRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_update_user_status(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let user = ctx.users.get(&Role::User).unwrap();

    let req: UpdateUserStatusRequest = UpdateUserStatusRequest {
        select_ids: vec![user.id],
        enable: false,
    };
    let (status, _resp) = ctx
        .app
        .api
        .update_user_status(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_forbidden_update_user_status(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let user = ctx.users.get(&Role::User).unwrap();

    let req: UpdateUserStatusRequest = UpdateUserStatusRequest {
        select_ids: vec![user.id],
        enable: false,
    };
    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();

    let (status, _resp) = ctx
        .app
        .api
        .update_user_status(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_update_user_status(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: UpdateUserStatusRequest = UpdateUserStatusRequest {
        select_ids: vec![],
        enable: false,
    };

    let (status, _resp) = ctx
        .app
        .api
        .update_user_status(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_update_deleted_user_status(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let deleted_user = ctx.users.get(&Role::DeletedUser).unwrap();

    let req: UpdateUserStatusRequest = UpdateUserStatusRequest {
        select_ids: vec![deleted_user.id],
        enable: false,
    };

    let (status, _resp) = ctx
        .app
        .api
        .update_user_status(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
}
