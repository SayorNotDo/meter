use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use fake::{Fake, Faker};
use server::constant::ACCESS_TOKEN_DECODE_KEY;
use server::dto::request::LoginRequest;
use server::utils::claim::UserClaims;
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_logout(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();
    let (status, _resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_invalid_token_logout(ctx: &mut SeedDbTestContext) {
    let invalid_token = Faker.fake::<String>();
    let (status, _resp) = ctx.app.api.logout(&invalid_token).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_already_logged_out(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();
    let (status, _resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
    assert!(status.is_success(), "status: {status}");

    let (status, _resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
    assert!(status.is_client_error(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_logout_session(ctx: &mut SeedDbTestContext) {
    let user = ctx.users.get(&Role::User).unwrap();
    let req = LoginRequest {
        username: user.username.clone(),
        password: user.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, _resp) = ctx.app.api.logout(&token.access_token).await.unwrap();
    assert!(status.is_success(), "status: {status}");

    /* TODO: check redis key whether still exist or not*/
    let user_claims = UserClaims::decode(&token.access_token, &ACCESS_TOKEN_DECODE_KEY).unwrap();
    assert!(
        server::service::session::check(&ctx.app.state.redis, &user_claims.claims)
            .await
            .is_err()
    )
}
