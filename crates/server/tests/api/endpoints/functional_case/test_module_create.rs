use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::dto::{
    request::{file::CreateModuleRequest, user::LoginRequest},
    response::CreateEntityResponse,
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
    ))
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
