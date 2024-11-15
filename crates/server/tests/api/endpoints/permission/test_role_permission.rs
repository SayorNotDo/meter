use crate::{assert_err, context::seeder::SeedDbTestContext, helper::user::Role};

use crate::helper::result::AppResponseResult;
use crate::helper::user::TestUser;
use server::dao::entity::{Permission, UserRolePermission};
use server::{dto::request::user::LoginRequest, errors::AppResponseError};
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
                .all(|item| { matches!(item, UserRolePermission { .. }) }),
            "resp: {resp:?}"
        );
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_admin_role_permission_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();
    let (status, resp) = ctx
        .app
        .api
        .get_role_permission(&token.access_token, ctx.project.id, admin.role_id)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    if let AppResponseResult::Ok(resp) = resp {
        assert!(!resp.is_empty(), "resp: {resp:?}");
        assert!(resp
            .iter()
            .all(|item| { matches!(item, Permission { .. }) }));
        assert_eq!(resp, admin.permission);
    };
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
