use fake::{Fake, Faker};
use server::{
    dao::{case::CaseDao, entity::FieldOption},
    dto::{
        request::{
            case::{CreateFieldRequest, DeleteFieldRequest, QueryFieldParam},
            user::LoginRequest,
        },
        response::CreateEntityResponse,
    },
};
use test_context::test_context;

use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_delete_text_field(ctx: &mut SeedDbTestContext) {
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
        let req: DeleteFieldRequest = DeleteFieldRequest { id: entity.id };

        let (status, _resp) = ctx
            .app
            .api
            .delete_field(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();
        assert!(status.is_success(), "status: {status}");

        let (status, _resp) = ctx
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

        assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
    }
}

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_delete_select_field(ctx: &mut SeedDbTestContext) {
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
        let req: DeleteFieldRequest = DeleteFieldRequest { id: entity.id };

        let (status, _resp) = ctx
            .app
            .api
            .delete_field(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();
        assert!(status.is_success(), "status: {status}");

        let (status, _resp) = ctx
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

        assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
        let client = ctx.app.state.pool.get().await.unwrap();
        let case_dao = CaseDao::new(&client);
        let options = case_dao.get_options_by_field_id(entity.id).await.unwrap();
        assert_eq!(options.len(), 0);
    }
}
