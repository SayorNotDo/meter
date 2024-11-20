use crate::{
    context::seeder::SeedDbTestContext,
    helper::{result::AppResponseResult, user::Role},
};

use server::{dao::entity::Field, dto::request::user::LoginRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_get_field_list(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };

    let token = ctx.app.api.get_token(&req).await.unwrap();

    let (status, resp) = ctx
        .app
        .api
        .get_field_list(&token.access_token, ctx.project.id, ctx.project.id)
        .await
        .unwrap();

    assert!(status.is_success(), "status: {status}");
    if let AppResponseResult::Ok(fields) = resp {
        assert!(fields.into_iter().all(|item| matches!(item, Field { .. })))
    }
}
