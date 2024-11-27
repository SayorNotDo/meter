use crate::{assert_err, context::seeder::SeedDbTestContext, helper::user::Role};
use server::{
    dto::request::{file::QueryModuleParam, user::LoginRequest},
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_case_modules(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, _resp) = ctx
        .app
        .api
        .get_case_module(
            &token.access_token,
            ctx.project.id,
            ctx.project.id,
            &QueryModuleParam { module_id: None },
        )
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_params_get_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let params: QueryModuleParam = QueryModuleParam {
        module_id: Some(100001),
    };

    let (status, resp) = ctx
        .app
        .api
        .get_case_module(&token.access_token, ctx.project.id, ctx.project.id, &params)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "MODULE_NOT_FOUND_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_path_parameter_get_modules(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_case_module(
            &token.access_token,
            ctx.project.id,
            10001,
            &QueryModuleParam { module_id: None },
        )
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "PROJECT_NOT_FOUND_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_path_parameter_get_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let param: QueryModuleParam = QueryModuleParam { module_id: Some(1) };
    let (status, resp) = ctx
        .app
        .api
        .get_case_module(&token.access_token, ctx.project.id, 10001, &param)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "PROJECT_NOT_FOUND_ERROR");
}
