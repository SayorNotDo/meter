use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::dto::{
    request::{
        case::{
            CreateFunctionalCaseRequest, FieldValue, SelectedField, UpdateFunctionalCaseRequest,
        },
        user::LoginRequest,
    },
    response::CreateEntityResponse,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_update_functional_case(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFunctionalCaseRequest = CreateFunctionalCaseRequest {
        name: Faker.fake::<String>(),
        module_id: 1,
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
    ));
    if let AppResponseResult::Ok(entity) = resp {
        let req: UpdateFunctionalCaseRequest = UpdateFunctionalCaseRequest {
            case_id: entity.id,
            module_id: 1,
            name: Faker.fake::<String>(),
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

        let (status, _resp) = ctx
            .app
            .api
            .update_functional_case(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();

        assert!(status.is_success(), "status: {status}");
    }
}
