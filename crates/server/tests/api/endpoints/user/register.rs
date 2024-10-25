use fake::{Fake, Faker};
use server::{
    dto::{
        request::{RegisterRequest, UserInfo},
        response::MessageResponse,
    },
    errors::AppResponseError,
};

use crate::{
    assert_err, assert_ok, context::state::TestContext, helper::result::AppResponseResult,
};

use test_context::test_context;

#[test_context(TestContext)]
#[tokio::test]
pub async fn test_success_register(ctx: &mut TestContext) {
    let mut user_list = Vec::new();
    let user: UserInfo = Faker.fake();
    user_list.push(user);
    let req: RegisterRequest = RegisterRequest {
        user_info_list: user_list,
    };
    let (status, resp) = ctx.api.register(&req).await.unwrap();
    assert!(matches!(
        resp,
        AppResponseResult::Ok(MessageResponse { .. })
    ));
    assert!(status.is_success(), "status: {status}");
    let (status, resp) = ctx.api.register(&req).await.unwrap();
    // assert_err!(resp, |e: &AppResponseError| e.message
    //     == "USER_ALREADY_EXISTS_ERROR");
    assert!(!status.is_success(), "status: {status}");
}
