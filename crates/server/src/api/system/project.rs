use std::f32::consts::E;
use axum::{Extension, Json};
use tracing::info;
use crate::errors::{AppResult, AppResponseError};
use crate::dto::response::ProjectInfoResponse;
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use crate::service::project;

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
) -> AppResult<Json(ProjectInfoResponse)> {
    match project::info().await {
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