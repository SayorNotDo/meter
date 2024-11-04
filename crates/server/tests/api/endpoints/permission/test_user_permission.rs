use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use server::{errors::AppResult, service::permission::check_user_permission};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_no_permission(ctx: &mut SeedDbTestContext) {
    let ordinary_user = ctx.users.get(&Role::User).unwrap();
    let uri: &str = "/api/test";
    let method: &str = "POST";
    let permission_result: AppResult<bool> = check_user_permission(
        &ctx.app.state,
        &ordinary_user.uuid,
        &ctx.project.id,
        uri,
        method,
    )
    .await;
    assert!(permission_result.is_err());
}
