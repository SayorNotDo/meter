use crate::{
    assert_err,
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};
use fake::{Fake, Faker};
use server::{
    dto::{
        request::{
            file::{CreateModuleRequest, DeleteModuleRequest, QueryModuleParam},
            user::LoginRequest,
        },
        response::CreateEntityResponse,
    },
    errors::AppResponseError,
};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_delete_case_module(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let req: CreateModuleRequest = CreateModuleRequest {
        name: Faker.fake::<String>(),
        project_id: ctx.project.id,
        parent_id: None,
    };

    let (status, resp) = ctx
        .app
        .api
        .create_case_module(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    assert!(matches!(
        resp,
        AppResponseResult::Ok(CreateEntityResponse { .. })
    ));

    if let AppResponseResult::Ok(entity) = resp {
        let req: DeleteModuleRequest = DeleteModuleRequest { id: entity.id };
        let (status, _resp) = ctx
            .app
            .api
            .delete_case_module(&token.access_token, ctx.project.id, &req)
            .await
            .unwrap();

        assert!(status.is_success(), "status: {status}");

        let (status, resp) = ctx
            .app
            .api
            .get_case_module(
                &token.access_token,
                ctx.project.id,
                ctx.project.id,
                &QueryModuleParam {
                    module_id: Some(entity.id),
                },
            )
            .await
            .unwrap();

        assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
        assert_err!(resp, |e: &AppResponseError| e.kind
            == "MODULE_NOT_FOUND_ERROR");
    }
}
