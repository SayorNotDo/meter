use crate::{
    context::{seeder::SeedDbTestContext, state::TestContext},
    helper::{
        result::AppResponseResult,
        user::{Role, TestUser},
    },
};
use fake::{Fake, Faker};

use server::dto::{
    request::{LoginRequest, RegisterRequest, UserInfo},
    response::MessageResponse,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_register(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let mut user_info_list = Vec::new();
    let user: UserInfo = Faker.fake();
    user_info_list.push(user);
    let admin_request = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&admin_request).await.unwrap();
    let req: RegisterRequest = RegisterRequest { user_info_list };
    let (status, resp) = ctx
        .app
        .api
        .register(&token.access_token, &req)
        .await
        .unwrap();
    assert!(matches!(
        resp,
        AppResponseResult::Ok(MessageResponse { .. })
    ));
    assert!(status.is_success(), "status: {status}");
    let (status, _) = ctx
        .app
        .api
        .register(&token.access_token, &req)
        .await
        .unwrap();
    assert!(!status.is_success(), "status: {status}");
}

#[test_context(TestContext)]
#[tokio::test]
pub async fn test_unauthorized_register(ctx: &mut TestContext) {
    let mut user_info_list = Vec::new();
    let user: UserInfo = Faker.fake();
    user_info_list.push(user);

    let req: RegisterRequest = RegisterRequest { user_info_list };

    let (status, _) = ctx.api.register("Bearer ", &req).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_forbidden_admin_register(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let mut user_info_list = Vec::new();
    let user: UserInfo = Faker.fake();
    user_info_list.push(user);

    let admin_request = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&admin_request).await.unwrap();

    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();

    let (status, _) = ctx
        .app
        .api
        .register(&token.access_token, &RegisterRequest { user_info_list })
        .await
        .unwrap();
    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_forbidden_user_register(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let ordinary_user = ctx.users.get(&Role::User).unwrap();
    TestUser::disable_user(&ctx.app.state.pool, ordinary_user.id)
        .await
        .unwrap();

    let mut user_info_list = Vec::new();
    let user: UserInfo = UserInfo {
        username: ordinary_user.username.clone(),
        email: ordinary_user.email.clone(),
    };
    user_info_list.push(user);
    let admin_request = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&admin_request).await.unwrap();
    let req: RegisterRequest = RegisterRequest { user_info_list };
    let (status, _) = ctx
        .app
        .api
        .register(&token.access_token, &req)
        .await
        .unwrap();
    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_permission_denied_register(ctx: &mut SeedDbTestContext) {
    let ordinary_user = ctx.users.get(&Role::User).unwrap();

    let mut user_info_list = Vec::new();
    let user: UserInfo = Faker.fake();
    user_info_list.push(user);

    let ordinary_user_request = LoginRequest {
        username: ordinary_user.username.clone(),
        password: ordinary_user.password.clone(),
    };
    let token = ctx.app.api.get_token(&ordinary_user_request).await.unwrap();

    let (status, _) = ctx
        .app
        .api
        .register(&token.access_token, &RegisterRequest { user_info_list })
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_batch_users_register(ctx: &mut SeedDbTestContext) {}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_batch_users_within_invalid_register(ctx: &mut SeedDbTestContext) {}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_batch_users_within_already_exists_register(ctx: &mut SeedDbTestContext) {}
