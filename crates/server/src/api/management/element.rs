use std::collections::HashMap;

use crate::{
    dto::{
        request::{CreateElementRequest, ElementQueryParam},
        response::{ElementResponse, FileModuleResponse, ListElementResoponse},
    },
    errors::AppResult,
    service::{element, file},
    state::AppState,
    utils::claim::UserClaims,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use tracing::{info, warn};

#[utoipa::path(
    post,
    path = "/management/element",
    request_body = CreateElementRequest,
    responses(
        (status = 200, description = "Element created", body = [ElementResponse])
    ),
    security(("jwt" = []))
)]
pub async fn create(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreateElementRequest>,
) -> AppResult {
    info!("controller layer create element with request: {request:?}");
    match element::create(&state, user.uid, request).await {
        Ok(_resp) => Ok(()),
        Err(e) => Err(e),
    }
}

pub async fn info(Extension(_state): Extension<AppState>) -> AppResult<Json<()>> {
    info!("controller layer query element information with params");
    Ok(Json(()))
}

#[utoipa::path(
    get,
    path = "/element/module/tree/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn tree(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<Vec<FileModuleResponse>>> {
    info!(
        "controller layer query element list with params: {}",
        project_id
    );
    match file::file_module_tree(&state, &project_id, "ELEMENT".into()).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => {
            warn!("Failed to get element module tree");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/element/list/:project_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn list(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<ListElementResoponse>> {
    info!(
        "controller layer query element list with params: {}",
        project_id
    );
    match element::list(&state, project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => {
            warn!("Failed to get element list");
            Err(e)
        }
    }
}

#[utoipa::path(
    get,
    path = "/management/element/count/:project_id",
    params(ElementQueryParam),
    responses(),
    security(("jwt" = []))
)]
pub async fn count(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<ElementQueryParam>,
) -> AppResult<Json<HashMap<String, i64>>> {
    info!("controller layer element count group by module in project: {project_id:?}");
    match element::count(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
