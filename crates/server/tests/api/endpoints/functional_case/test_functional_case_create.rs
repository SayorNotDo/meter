use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::dto::{
    request::{case::CreateFunctionalCaseRequest, user::LoginRequest},
    response::CreateEntityResponse,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_functional_case(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFunctionalCaseRequest = CreateFunctionalCaseRequest {
        name: Faker.fake::<String>(),
        module_id: 0,
        template_id: 1,
        tags: Some(Faker.fake::<String>()),
        fields: vec![],
    };

    let (status, resp) = ctx
        .app
        .api
        .create_functional_case(&token.access_token, ctx.project.id, &req)
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
pub async fn test_invalid_params_create_functional_case(_ctx: &mut SeedDbTestContext) {}
