use crate::{assert_err, context::seeder::SeedDbTestContext, helper::user::Role};

use crate::helper::result::AppResponseResult;
use crate::helper::user::TestUser;
use server::{
    dto::{request::LoginRequest, response::UriPermission},
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_get_role_permission_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");
    if let AppResponseResult::Ok(resp) = resp {
        assert!(!resp.is_empty(), "resp: {resp:?}");
        assert!(
            resp.iter()
                .all(|item| { matches!(item, UriPermission { .. }) }),
            "resp: {resp:?}"
        );
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_access_denied_get_role_permission_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    TestUser::disable_user(&ctx.app.state.pool, admin.id)
        .await
        .unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission_list(&token.access_token, ctx.project.id)
        .await
        .unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind == "FORBIDDEN_ERROR");
}
