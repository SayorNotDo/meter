use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::result::AppResponseResult,
    helper::user::{Role, TestUser},
};

use reqwest::StatusCode;

use server::{
    dto::{
        request::{user::LoginRequest, DeleteRoleRequest},
        response::MessageResponse,
    },
    errors::AppResponseError,
};
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
    let req: DeleteRoleRequest = DeleteRoleRequest {
        ids: vec![role.role_id],
    };

    let (status, resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(MessageResponse { .. })
    ));

    let (status, resp) = ctx
        .app
        .api
        .get_user_role(&token.access_token, ctx.project.id, role.role_id)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "ROLE_NOT_FOUND_ERROR");

    let (status, _resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_input_delete_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let req: DeleteRoleRequest = DeleteRoleRequest { ids: vec![] };

    let (status, _resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_delete_internal_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let req: DeleteRoleRequest = DeleteRoleRequest {
        ids: vec![admin.role_id],
    };

    let (status, _resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_access_denied_delete_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let user = ctx.users.get(&Role::User).unwrap();

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let req: DeleteRoleRequest = DeleteRoleRequest {
        ids: vec![user.role_id],
    };

    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();

    let (status, _resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_delete_allocated_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let user = ctx.users.get(&Role::User).unwrap();

    let req: DeleteRoleRequest = DeleteRoleRequest {
        ids: vec![user.role_id],
    };

    let (status, _resp) = ctx
        .app
        .api
        .delete_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
}
