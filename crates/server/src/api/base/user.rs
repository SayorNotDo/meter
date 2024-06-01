use crate::errors::AppResult;
use crate::dto::response::UserInfoResponse;


#[utoipa::path(
    get,
    path = "/user/info",
    responses(
        (status = 200, description = "Get user info", body = [UserInfoResponse]),
        (status = 401, description = "User Unauthorized", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
)]
pub async fn info() -> AppResult<UserInfoResponse> {
    Ok(UserInfoResponse {})
}