use crate::{
    dto::{
        request::{UserDeleteRequest, UserStatusRequest},
        response::ListUserResponse,
    },
    errors::AppResult,
    service,
    state::AppState,
    utils::claim::UserClaims,
};
use axum::{Extension, Json};
use tracing::info;

#[utoipa::path(
    get,
    path = "/user/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<ListUserResponse>> {
    info!("controller layer get user list");
    match service::user::list(&state, user.uid).await {
        Ok(resp) => Ok(Json(ListUserResponse { list: resp })),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    put,
    path = "/user/status",
    responses(),
    security(("jwt" = []))
)]
pub async fn update_status(
    Extension(state): Extension<AppState>,
    _user: UserClaims,
    Json(request): Json<UserStatusRequest>,
) -> AppResult {
    info!("controller layer update user status with request: {request:?}");
    match service::user::update_status(&state, request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    delete,
    path = "/user",
    request_body = UserDeleteRequest,
    responses(),
    security(("jwt" = []))
)]
pub async fn delete(
    Extension(state): Extension<AppState>,
    Json(request): Json<UserDeleteRequest>,
) -> AppResult {
    info!("controller layer delete user with ids: {request:?}");
    match service::user::batch_delete(&state, request.ids).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
