use crate::{dao::entity::ProjectMember, entity::project::ProjectInfo};

use axum::extract::Path;
use axum::{Extension, Json};
use tracing::info;

use crate::dto::response::{MessageResponse, ProjectInfoResponse};
use crate::errors::{AppResponseError, AppResult};
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
    path = "/management/project",
    responses(
        (status = 200, description = "Success get project list", body = [ProjectInfo]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Project not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_project_list(
    Extension(state): Extension<AppState>,
    user: UserClaims,
) -> AppResult<Json<Vec<ProjectInfo>>> {
    info!("project controller layer get project list by user: {user:?}");
    match project::list(&state, user.uid).await {
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
pub async fn members(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<Vec<ProjectMember>>> {
    match project::members(&state, &project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
