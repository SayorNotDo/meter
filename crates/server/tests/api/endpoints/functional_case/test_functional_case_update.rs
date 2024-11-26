use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use fake::{Fake, Faker};
use server::dto::request::{
    case::{FieldValue, SelectedField, UpdateFunctionalCaseRequest},
    user::LoginRequest,
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

    let req: UpdateFunctionalCaseRequest = UpdateFunctionalCaseRequest {
        case_id: 1,
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
