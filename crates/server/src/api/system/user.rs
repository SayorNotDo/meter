use crate::dao::entity::UserRole;
use crate::dto::response::CreateEntityResponse;
use crate::{
    dao::entity::UserRolePermission,
    dto::{
        request::{CreateRoleRequest, UserDeleteRequest, UserQueryParam, UserStatusRequest},
        response::{ListUserResponse, MessageResponse},
    },
    errors::{AppResponseError, AppResult},
    service,
    state::AppState,
    utils::claim::UserClaims,
};
use axum::extract::Path;
use axum::{Extension, Json};
use garde::Validate;
use tracing::info;

#[utoipa::path(
    post,
    path = "/system/role",
    request_body = CreateRoleRequest,
    responses(
        (status = 200, description = "Success create user role", body = [MessageResponse]),
        (status = 400, description = "Invalid parameters", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 403, description = "Forbidden", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn create_role(
    Extension(state): Extension<AppState>,
    _user: UserClaims,
    Json(request): Json<CreateRoleRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    request.validate()?;
    match service::user::create_role(&state, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(err) => Err(err),
    }
}

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
    match service::user::list(&state, user.uid, UserQueryParam { idle: false }).await {
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
    user: UserClaims,
    Json(request): Json<UserDeleteRequest>,
) -> AppResult {
    info!("controller layer delete user with ids: {request:?}");
    match service::user::batch_delete(&state, user.uid, request.ids).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/system/role/permission/:role_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn role_permission(
    Extension(_state): Extension<AppState>,
    Path(_role_id): Path<i32>,
) -> AppResult<Json<UserRolePermission>> {
    Ok(Json(UserRolePermission {
        user_role: UserRole {
            id: 0,
            name: "".to_string(),
            role_type: "".to_string(),
            internal: false,
            created_at: Default::default(),
            created_by: "".to_string(),
            updated_at: None,
            description: None,
        },
        permission_list: vec![],
    }))
}

#[utoipa::path(
    get,
    path = "/role/permission/list",
    responses(
        (status = 200, description = "Get role permission list", body = [Vec<UserRolePermission>]),
        (status = 400, description = "role permission list not found", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn role_permission_list(
    Extension(state): Extension<AppState>,
) -> AppResult<Json<Vec<UserRolePermission>>> {
    match service::permission::get_role_permission_list(&state).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
