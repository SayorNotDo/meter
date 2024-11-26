use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use server::dto::{
    request::{user::LoginRequest, ListQueryParam},
    response::case::FunctionalCaseResponse,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_functional_case(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_functional_case(&token.access_token, ctx.project.id, 1)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(FunctionalCaseResponse { .. })
    ))
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_functional_case_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, _resp) = ctx
        .app
        .api
        .get_functional_case_list(&token.access_token, ctx.project.id, &None)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_functional_case_list_with_query(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let params: Option<ListQueryParam> = Some(ListQueryParam {
        module_id: Some(1),
        page_size: Some(10),
        page_token: None,
    });

    let (status, _resp) = ctx
        .app
        .api
        .get_functional_case_list(&token.access_token, ctx.project.id, &params)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}
