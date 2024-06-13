use axum::{Extension, Json};
use axum::extract::{Path, Query};

use crate::dto::{response::FileModuleResponse, request::QueryTemplateFieldParam};
use crate::errors::AppResult;
use crate::state::AppState;
use tracing::info;
use crate::dto::response::CaseInfoResponse;
use crate::service::case;

#[utoipa::path(
    get,
    path = "/case/module/tree/:project_id",
    responses(
        (status = 200, description = "Get case tree", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case tree not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn tree(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<Vec<FileModuleResponse>>> {
    info!("case module tree query param: {project_id:?}");
    match case::module_tree(&state, &project_id).await {
        Ok(resp) => {
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Failed to get case module tree");
            Err(e)
        }
    }
}


#[utoipa::path(
    get,
    path = "/case/template/field/:project_id",
    params(QueryTemplateFieldParam),
    responses(),
    security(("jwt" = []))
)]
pub async fn template_field(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<QueryTemplateFieldParam>,
) -> AppResult<Json<Vec<()>>> {
    info!("case template field query param: {param:?}, project_id: {project_id:?}");
    Ok(Json(vec![]))
}

#[utoipa::path(
    get,
    path = "/case/list/:project_id",
    responses(
        (status = 200, description = "Get case list", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case list not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))

)]
pub async fn list() -> AppResult<Json<Vec<CaseInfoResponse>>> {
    Ok(Json(vec![]))
}