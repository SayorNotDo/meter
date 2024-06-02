use axum::{Extension, Json};
use axum::extract::Path;
use tracing::info;

use crate::dto::response::ProjectInfoResponse;
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
    Path(project_id): Path<String>,
) -> AppResult<Json<ProjectInfoResponse>> {
    info!("Project info with path param: {project_id:?}");
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