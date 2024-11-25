use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dto::{
        request::{
            case::{CreateFunctionalCaseRequest, FieldValue, SelectedField},
            user::LoginRequest,
        },
        response::CreateEntityResponse,
    },
    errors::AppResponseError,
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
        fields: vec![
            SelectedField {
                id: 1,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
            SelectedField {
                id: 2,
                value: FieldValue::Select(1),
            },
            SelectedField {
                id: 3,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
            SelectedField {
                id: 4,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
            SelectedField {
                id: 5,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
            SelectedField {
                id: 6,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
            SelectedField {
                id: 7,
                value: FieldValue::Text(Faker.fake::<String>()),
            },
        ],
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
pub async fn test_invalid_req_create_functional_case(ctx: &mut SeedDbTestContext) {
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

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "INVALID_INPUT_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_conflict_value_option_create_function_case(ctx: &mut SeedDbTestContext) {
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
        fields: vec![SelectedField {
            id: 1,
            value: FieldValue::Select(1),
        }],
    };

    let (status, resp) = ctx
        .app
        .api
        .create_functional_case(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "BAD_REQUEST_ERROR");
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_conflict_value_text_create_function_case(ctx: &mut SeedDbTestContext) {
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
        fields: vec![SelectedField {
            id: 3,
            value: FieldValue::Select(1),
        }],
    };

    let (status, resp) = ctx
        .app
        .api
        .create_functional_case(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    assert_err!(resp, |e: &AppResponseError| e.kind == "BAD_REQUEST_ERROR");
}
