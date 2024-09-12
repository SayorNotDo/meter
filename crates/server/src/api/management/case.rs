use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::{Extension, Json};
use axum_extra::extract::WithRejection;

use crate::dao::entity::CustomField;
use crate::dto::request::{
    CaseQueryParam, CreateFunctionalCaseRequest, CreateModuleRequest, DiagnoseRequest,
    IssueRelationRequest,
};
use crate::dto::response::{
    CaseDetailResponse, CreateScriptResponse, DiagnoseResponse, ListCaseResponse, TemplateResponse,
};
use crate::dto::{
    request::{CreateScriptRequest, ListQueryParam, QueryTemplateParam},
    response::{FileModuleResponse, RequirementInfoResponse},
};
use crate::errors::{AppError, AppResult};
use crate::service::{self, case, file};
use crate::state::AppState;
use crate::utils::claim::UserClaims;
use tracing::info;

#[utoipa::path(
    get,
    path = "/case/module/tree/:project_id",
    responses(
        (status = 200, description = "Get case tree", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn tree(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
) -> AppResult<Json<Vec<FileModuleResponse>>> {
    info!("case module tree query param: {project_id:?}");
    match file::file_module_tree(&state, &project_id, "CASE".into()).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => {
            info!("Failed to get case module tree");
            Err(e)
        }
    }
}

#[utoipa::path(
    post,
    path = "/management/case/module",
    responses(),
    security(("jwt" = []))
)]
pub async fn create_module(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreateModuleRequest>,
) -> AppResult {
    info!("case module create with request: {request:?}");
    match file::create_file_module(
        &state,
        user.uid,
        &request.project_id,
        "CASE".into(),
        request.parent_id,
        &request.name,
    )
    .await
    {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    delete,
    path = "/management/case/module/:module_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn delete_module(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Path(module_id): Path<i32>,
) -> AppResult {
    info!("delete case module with module_id: {module_id}");
    Ok(())
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
    post,
    path = "/management/case/functional_case",
    request_body = CreateFunctionalCaseRequest,
    responses(),
    security(("jwt" = []))
)]
pub async fn create_functional_case(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreateFunctionalCaseRequest>,
) -> AppResult<Json<()>> {
    info!("create functional case with request: {request:?}");
    match case::create_functional_case(&state, user.uid, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "case/functional-case/:case_id",
    responses(),
    security(("jwt" = []))
)]
pub async fn get_functional_case(
    Extension(state): Extension<AppState>,
    _user: UserClaims,
    Path(case_id): Path<i32>,
) -> AppResult<Json<CaseDetailResponse>> {
    info!("query functional case with path param: {case_id:?}");
    match case::get_functional_case(&state, case_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "case/functional-case/related-issue",
    request_body = IssueRelationRequest,
    responses(),
    security(("jwt" = []))
)]
pub async fn create_issue_relation(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<IssueRelationRequest>,
) -> AppResult<()> {
    match case::create_issue_relation(&state, user.uid, request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/field/:project_id",
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
    path = "/management/case/list/:project_id",
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
    info!("controller layer case list proejct_id: {project_id:?} query with param: {param:?}");
    match case::list(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/case/count/:project_id",
    params(CaseQueryParam),
    responses(
        (status = 200, description = "Get case module info", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case module info not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn count(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(param): Query<CaseQueryParam>,
) -> AppResult<Json<HashMap<String, i64>>> {
    info!("controller layer case count group by module in project: {project_id:?}");
    match case::count(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/case/detail/:case_id",
    params(),
    responses(
        (status = 200, description = "Get case details", body = [()]),
        (status = 404, description = "Case not found", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn detail(
    Extension(state): Extension<AppState>,
    Path(case_id): Path<i32>,
) -> AppResult<Json<CaseDetailResponse>> {
    info!("controller layer case detail with id: {case_id:?}");
    match case::detail(&state, &case_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/management/case/script/generate",
    request_body=CreateScriptRequest,
    responses()
)]
pub async fn create_script(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    WithRejection(Json(request), _): WithRejection<Json<CreateScriptRequest>, AppError>,
) -> AppResult<Json<CreateScriptResponse>> {
    info!("controller layer create script with request: {request:?}");
    match service::case::gen_script(&state, user.uid, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/management/case/environment/diagnose",
    request_body=DiagnoseRequest,
    responses()
)]
pub async fn env_diagnose(
    Extension(state): Extension<AppState>,
    WithRejection(Json(request), _): WithRejection<Json<DiagnoseRequest>, AppError>,
) -> AppResult<Json<DiagnoseResponse>> {
    info!("controller layer diagnose environment with request");
    match service::case::env_diagnose(&state, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}
