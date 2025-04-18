use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    Extension, Json,
};

use crate::{
    dto::{
        request::{
            file::{CreateModuleRequest, QueryModuleParam},
            CreatePlanRequest, ListQueryParam, PlanQueryParam,
        },
        response::{CreateEntityResponse, FileModuleResponse, ListPlanResponse},
    },
    entity::file::ModuleType,
    errors::{AppResponseError, AppResult},
    service::{file, plan},
    state::AppState,
    utils::{claim::UserClaims, header::extract_project_id},
};

use tracing::info;

#[utoipa::path(
    post,
    path = "/management/plan",
    request_body = CreatePlanRequest,
    responses()
)]
pub async fn create(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreatePlanRequest>,
) -> AppResult {
    info!("controller layer create plan with request: {request:?}");
    match plan::create(&state, user.uid, request).await {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/test-plan/module/tree/:project_id",
    responses()
)]
pub async fn tree(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(params): Query<QueryModuleParam>,
) -> AppResult<Json<Vec<FileModuleResponse>>> {
    info!("controller layer query with param: {project_id:?}");
    match file::get_file_module(&state, &project_id, ModuleType::Plan, params).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => {
            info!("Failed to get plan module tree");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/management/test-plan/module/count/:project_id",
    params(PlanQueryParam),
    responses(
        (status = 200, description = "Get case module info"),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case module info not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn count(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<PlanQueryParam>,
) -> AppResult<Json<HashMap<String, i64>>> {
    info!("controller layer case count group by module in project: {project_id:?}");
    match plan::count(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/test-plan/list/:project",
    responses(),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    _user: UserClaims,
    Path(project_id): Path<i32>,
    Query(param): Query<ListQueryParam>,
) -> AppResult<Json<ListPlanResponse>> {
    info!("controller layer plan list query with project_id: {project_id:?}");
    match plan::get_plan_list(&state, &project_id, param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/management/test-plan/module",
    request_body = CreateModuleRequest,
    responses(),
    security(("jwt" = []))
)]
pub async fn create_module(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    headers: HeaderMap,
    Json(request): Json<CreateModuleRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    info!("controller layer create module with body: {request:?}");
    let project_id = extract_project_id(&headers)?;
    match file::create_file_module(&state, project_id, user.uid, ModuleType::Plan, &request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
