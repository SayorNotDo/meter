use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dao::entity::Permission,
    dto::{
        request::{CreateRoleRequest, LoginRequest},
        response::CreateEntityResponse,
    },
    errors::AppResponseError,
};
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
    let (status, resp) = ctx
        .app
        .api
        .create_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(CreateEntityResponse { .. })
    ));

    if let AppResponseResult::Ok(CreateEntityResponse { id: role_id }) = resp {
        let (status, resp) = ctx
            .app
            .api
            .get_role_permission(&token.access_token, ctx.project.id, role_id)
            .await
            .unwrap();

        assert!(status.is_success(), "role permission: {status}");
        if let AppResponseResult::Ok(permission_list) = resp {
            assert!(permission_list
                .iter()
                .all(|item| matches!(item, Permission { .. })));
            assert_ne!(permission_list.len(), 0);
        }
    }

    let (status, resp) = ctx
        .app
        .api
        .create_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| e.kind
        == "ROLE_ALREADY_EXISTS_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_user_role_permission(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();
    let role = ctx.users.get(&Role::User).unwrap();
    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_role_permission(&token.access_token, ctx.project.id, role.role_id)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    if let AppResponseResult::Ok(permission_list) = resp {
        assert!(permission_list
            .iter()
            .all(|item| matches!(item, Permission { .. })));
    }
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

    let (status, resp) = ctx
        .app
        .api
        .create_role(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_client_error(), "status: {status}");
    assert_err!(resp, |e: &AppResponseError| {
        e.kind == "INVALID_INPUT_ERROR"
    });
}
