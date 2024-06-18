use axum::extract::{Path, Query};
use axum::{Extension, Json};

use crate::dao::entity::CustomField;
use crate::dto::response::{ListCaseResponse, TemplateResponse};
use crate::dto::{
    request::{ListQueryParam, QueryTemplateParam},
    response::{FileModuleResponse, RequirementInfoResponse},
};
use crate::errors::AppResult;
use crate::service::{case, file};
use crate::state::AppState;
use tracing::info;

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
    match file::file_module_tree(&state, &project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => {
            info!("Failed to get case module tree");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/case/template/:project_id",
    params(QueryTemplateParam),
    responses(),
    security(("jwt" = []))
)]
pub async fn template(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<QueryTemplateParam>,
) -> AppResult<Json<TemplateResponse>> {
    info!("case template query param: {param:?}, project_id: {project_id:?}");
    match case::template(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/field/:priject_id",
    params(QueryTemplateParam),
    responses(),
    security(("jwt" = []))
)]
pub async fn field(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<QueryTemplateParam>,
) -> AppResult<Json<Vec<CustomField>>> {
    info!("case template field query param: {param:?}, project_id: {project_id:?}");
    match case::field(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/info/requirement/:project_id",
    responses(
        (status = 200, description = "Get related pr info", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case tree not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn info(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<RequirementInfoResponse>> {
    info!("case related information with project_id: {project_id:?}");
    match case::info(&state, &project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/list/:project_id",
    params(ListQueryParam),
    responses(
        (status = 200, description = "Get case list", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case list not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<ListQueryParam>,
) -> AppResult<Json<ListCaseResponse>> {
    info!("case list query with param: {param:?}");
    match case::list(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
