use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::user::{Role, TestUser},
    unwrap,
};
use fake::{Fake, Faker};
use server::{
    dto::{request::user::LoginRequest, response::LoginResponse},
    errors::AppResponseError,
};

use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_login(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
    let resp = unwrap!(resp);
    assert!(status.is_success(), "status: {status}");
    match resp {
        LoginResponse::Token(token) => {
            assert!(!token.access_token.is_empty());
            assert!(!token.refresh_token.is_empty());
        }
        LoginResponse::Code { .. } => {
            panic!("not expected to receive message.");
        }
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_forbidden_login(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    TestUser::disable_user(&ctx.app.state.pool, user.id)
        .await
        .unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind == "FORBIDDEN_ERROR");
    TestUser::enable_user(&ctx.app.state.pool, user.id)
        .await
        .unwrap()
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_wrong_password_login(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: (9..20).fake::<String>(),
    };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind == "BAD_REQUEST_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_fail_login(ctx: &mut SeedDbTestContext) {
    let username = Faker.fake::<String>();
    let password = (9..20).fake::<String>();
    let req = LoginRequest { username, password };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "USER_NOT_FOUND_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_short_password_login(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: (0..7).fake::<String>(),
    };
    let (status, resp) = ctx.app.api.login(&req).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind == "INVALID_INPUT_ERROR");
}
