use crate::dto::response::UserInfoResponse;
use crate::errors::AppResult;
use crate::service;
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use crate::{dao::entity::UserRoleOption, dto::request::UserInfoUpdateRequest};
use axum::{extract::Path, Extension, Json};
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

#[utoipa::path(
    put,
    path = "/user/info",
    request_body = UserInfoUpdateRequest,
    responses(),
    security(("jwt" = []))
)]
pub async fn update(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<UserInfoUpdateRequest>,
) -> AppResult {
    info!("controller layer update user info with request: {request:?}");
    match service::user::update_info(&state, user.uid, request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/user/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(_state): Extension<AppState>,
    Path(_project_id): Path<i32>,
    _user: UserClaims,
) -> AppResult<()> {
    info!("controller layer get user list");
    Ok(())
}

#[utoipa::path(
    get,
    path = "/user/role/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn role_list(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    _user: UserClaims,
) -> AppResult<Json<Vec<UserRoleOption>>> {
    info!("controller layer get user role list with project_id: {project_id}");
    match service::user::role_list(&state, project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
