use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dao::entity::Field,
    dto::{
        request::{
            case::{CreateFieldRequest, QueryFieldParam},
            user::LoginRequest,
        },
        response::CreateEntityResponse,
    },
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_create_field(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateFieldRequest = CreateFieldRequest {
        name: Faker.fake::<String>(),
        field_type: "TEXT".to_string(),
        project_id: ctx.project.id,
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
