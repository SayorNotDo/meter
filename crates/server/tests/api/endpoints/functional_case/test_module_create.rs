use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dto::{
        request::{
            file::{CreateModuleRequest, QueryModuleParam},
            user::LoginRequest,
        },
        response::{CreateEntityResponse, FileModuleResponse},
    },
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_root_case_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateModuleRequest = CreateModuleRequest {
        name: Faker.fake::<String>(),
        project_id: ctx.project.id,
        parent_id: None,
    };

    let (status, resp) = ctx
        .app
        .api
        .create_case_module(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(CreateEntityResponse { .. })
    ));
    if let AppResponseResult::Ok(entity) = resp {
        let params = QueryModuleParam {
            module_id: Some(entity.id),
        };
        let (status, resp) = ctx
            .app
            .api
            .get_case_module(&token.access_token, ctx.project.id, ctx.project.id, &params)
            .await
            .unwrap();

        assert!(status.is_success(), "status: {status}");
        if let AppResponseResult::Ok(modules) = resp {
            assert_eq!(modules.len(), 1);
            assert!(modules
                .into_iter()
                .all(|item| matches!(item, FileModuleResponse { .. })))
        }
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_req_create_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateModuleRequest = CreateModuleRequest {
        name: Faker.fake::<String>(),
        project_id: ctx.project.id,
        parent_id: Some(1001),
    };

    let (status, resp) = ctx
        .app
        .api
        .create_case_module(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "BAD_REQUEST_ERROR");

    let req: CreateModuleRequest = CreateModuleRequest {
        name: Faker.fake::<String>(),
        project_id: 1001,
        parent_id: None,
    };
    let (status, resp) = ctx
        .app
        .api
        .create_case_module(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
    assert_err!(resp, |e: &AppResponseError| e.kind == "FORBIDDEN_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_create_case_module_ordering(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateModuleRequest = CreateModuleRequest {
        name: Faker.fake::<String>(),
        project_id: ctx.project.id,
        parent_id: None,
    };

    let (status, _resp) = ctx
        .app
        .api
        .create_case_module(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}
