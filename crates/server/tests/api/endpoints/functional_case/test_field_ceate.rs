use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dto::{
        request::{
            case::{CreateFieldRequest, QueryFieldParam},
            user::LoginRequest,
        },
        response::CreateEntityResponse,
    },
    entity::case::{Field, FieldOption},
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_text_field(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFieldRequest = CreateFieldRequest {
        name: Faker.fake::<String>(),
        field_type: "TEXT".to_string(),
        remark: Some(Faker.fake::<String>()),
        options: None,
    };
    let (status, resp) = ctx
        .app
        .api
        .create_field(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(CreateEntityResponse { .. })
    ));
    if let AppResponseResult::Ok(entity) = resp {
        let (status, resp) = ctx
            .app
            .api
            .get_field_list(
                &token.access_token,
                ctx.project.id,
                ctx.project.id,
                &QueryFieldParam {
                    field_id: Some(entity.id),
                },
            )
            .await
            .unwrap();

        assert!(status.is_success(), "status : {status}");
        if let AppResponseResult::Ok(field_list) = resp {
            assert!(field_list
                .into_iter()
                .all(|item| matches!(item, Field { .. })))
        }
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_select_field(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFieldRequest = CreateFieldRequest {
        name: Faker.fake::<String>(),
        field_type: "SELECT".to_string(),
        remark: Some(Faker.fake::<String>()),
        options: Some(vec![FieldOption {
            id: 0,
            value: "test".to_string(),
            position: 0,
        }]),
    };

    let (status, _resp) = ctx
        .app
        .api
        .create_field(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_empty_vec_create_select_field(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFieldRequest = CreateFieldRequest {
        name: Faker.fake::<String>(),
        field_type: "SELECT".to_string(),
        remark: Some(Faker.fake::<String>()),
        options: Some(vec![]),
    };
    let (status, resp) = ctx
        .app
        .api
        .create_field(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "INVALID_INPUT_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_null_options_create_select_field(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFieldRequest = CreateFieldRequest {
        name: Faker.fake::<String>(),
        field_type: "SELECT".to_string(),
        remark: Some(Faker.fake::<String>()),
        options: None,
    };
    let (status, resp) = ctx
        .app
        .api
        .create_field(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "BAD_REQUEST_ERROR");
}
