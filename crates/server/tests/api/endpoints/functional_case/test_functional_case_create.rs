use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use fake::{Fake, Faker};
use server::dto::request::{user::LoginRequest, CreateFunctionalCaseRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFunctionalCaseRequest = CreateFunctionalCaseRequest {
        name: Faker.fake::<String>(),
        module_id: 0,
        template_id: 0,
        tags: Some(Faker.fake::<String>()),
        custom_fields: vec![],
    };

    let (status, _resp) = ctx
        .app
        .api
        .create_functional_case(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}
