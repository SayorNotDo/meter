use crate::dao::entity::ProjectMember;
use crate::dao::project::ProjectInfo;
use axum::extract::{Path, Query};
use axum::{Extension, Json};
use tracing::info;

use crate::dto::request::ProjectQueryParam;
use crate::dto::response::{MessageResponse, ProjectInfoResponse};
use crate::errors::AppResult;
use crate::service::project;
use crate::state::AppState;
use crate::utils::claim::UserClaims;

#[utoipa::path(
    get,
    path = "/project/:project_id",
    responses(
        (status = 200, description = "Get project info", body = [ProjectInfoResponse]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Project not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn info(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<ProjectInfoResponse>> {
    info!("Project info with path param: {project_id:?}");
    match project::info(&state, project_id).await {
        Ok(resp) => {
            info!("Get Project info successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Failed to get project information");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/project/list",
    params(ProjectQueryParam),
    responses(
        (status = 200, description = "Get project list", body = [ProjectListResponse]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Project not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Query(param): Query<ProjectQueryParam>,
) -> AppResult<Json<Vec<ProjectInfo>>> {
    info!("Project list with path param: {param:?}");
    match project::list(&state, user.uid, param.organization_id).await {
        Ok(resp) => {
            info!("Get Project list successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Failed to get project list");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/project/has-permission/:project_id",
    responses(
        (status = 200, description = "Get project list", body = [MessageResponse]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Project not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn permission(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Path(project_id): Path<i32>,
) -> AppResult<Json<MessageResponse>> {
    info!("Project list with path param: {:?}", user.uid);
    match project::permission(&state, project_id, user.uid).await {
        Ok(resp) => {
            info!("Get Project list successfully.");
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Failed to get project list");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/project/member/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn member(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<Vec<ProjectMember>>> {
    match project::member(&state, &project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
