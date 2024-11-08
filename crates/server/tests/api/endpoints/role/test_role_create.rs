use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use fake::{Fake, Faker};
use server::dto::request::{CreateRoleRequest, LoginRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();
    let req: CreateRoleRequest = CreateRoleRequest {
        name: Faker.fake::<String>(),
        description: Some(Faker.fake::<String>()),
        permission_list: vec![1],
    };
    let (status, _resp) = ctx
        .app
        .api
        .create_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission(&token.access_token, ctx.project.id, 1)
        .await
        .unwrap();
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_create_no_permission_role(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let req = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateRoleRequest = CreateRoleRequest {
        name: Faker.fake::<String>(),
        description: Some(Faker.fake::<String>()),
        permission_list: vec![],
    };

    let (status, _resp) = ctx
        .app
        .api
        .create_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
}
