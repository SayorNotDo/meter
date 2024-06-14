use crate::dto::response::UserInfoResponse;
use crate::errors::AppResult;
use crate::service;
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use axum::{Extension, Json};
use tracing::{info, warn};

#[utoipa::path(
    get,
    path = "/user/info",
    responses(
        (status = 200, description = "Get user info", body = [UserInfoResponse]),
        (status = 401, description = "User Unauthorized", body = [AppResponseError]),
        (status = 404, description = "User not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn info(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<UserInfoResponse>> {
    info!("User info with request: {:?}", user);
    match service::user::info(&state, user.uid).await {
        Ok(resp) => {
            info!("get user info successfully");
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Failed to get user info: {e:?}");
            Err(e)
        }
    }
}

#[utoipa::path(get, path = "/user/list", responses())]
pub async fn list() -> AppResult<()> {
    Ok(())
}
