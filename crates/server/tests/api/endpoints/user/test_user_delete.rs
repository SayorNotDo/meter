use crate::helper::user::TestUser;
use crate::{context::seeder::SeedDbTestContext, helper::user::Role};
use server::dto::request::{DeleteUserRequest, LoginRequest};
use test_context::test_context;

#[test_context(SeedDbTestContext)]
#[tokio::test]
pub async fn test_success_delete_user(ctx: &mut SeedDbTestContext) {
    let admin = ctx.users.get(&Role::Admin).unwrap();

    let req: LoginRequest = LoginRequest {
        username: admin.username.clone(),
        password: admin.password.clone(),
    };
    let user = ctx.users.get(&Role::User).unwrap();
    let token = ctx.app.api.get_token(&req).await.unwrap();
    let req: DeleteUserRequest = DeleteUserRequest { ids: vec![user.id] };

    TestUser::disable_user(&ctx.app.state.pool, user.id)
        .await
        .unwrap();

    let (status, _resp) = ctx
        .app
        .api
        .delete_user(&token.access_token, ctx.project.id, &req)
        .await
        .unwrap();
    assert!(status.is_success(), "status: {status}")
}
