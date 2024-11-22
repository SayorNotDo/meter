use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};

use server::dto::{
    request::{
        case::{CreateFieldRequest, QueryFieldParam, UpdateFieldRequest},
        user::LoginRequest,
    },
    response::CreateEntityResponse,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_update_field(ctx: &mut SeedDbTestContext) {
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

    assert!(status.is_success(), "status : {status}");
    if let AppResponseResult::Ok(entity) = resp {
        let req: UpdateFieldRequest = UpdateFieldRequest {
            id: entity.id,
            name: Faker.fake::<String>(),
            field_type: "TEXT".to_string(),
            remark: Some(Faker.fake::<String>()),
            options: None,
        };

        let (status, _resp) = ctx
            .app
            .api
            .update_field(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();

        assert!(status.is_success(), "status: {status}");

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

        assert!(status.is_success(), "status: {status}");
        if let AppResponseResult::Ok(fields) = resp {
            for item in fields.into_iter() {
                assert_eq!(req.name, item.name);
                assert_eq!(req.field_type, item.field_type);
            }
        }
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_empty_vec_update_text2select_field(ctx: &mut SeedDbTestContext) {
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
        let req: UpdateFieldRequest = UpdateFieldRequest {
            id: entity.id,
            name: Faker.fake::<String>(),
            field_type: "SELECT".to_string(),
            remark: Some(Faker.fake::<String>()),
            options: Some(vec![]),
        };

        let (status, _resp) = ctx
            .app
            .api
            .update_field(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_null_vec_update_text2select_field(ctx: &mut SeedDbTestContext) {
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
        let req: UpdateFieldRequest = UpdateFieldRequest {
            id: entity.id,
            name: Faker.fake::<String>(),
            field_type: "SELECT".to_string(),
            remark: Some(Faker.fake::<String>()),
            options: None,
        };

        let (status, _resp) = ctx
            .app
            .api
            .update_field(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::BAD_REQUEST);
    }
}
