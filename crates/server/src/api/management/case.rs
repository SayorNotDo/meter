use std::collections::HashMap;

use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    Extension, Json,
};
use axum_extra::extract::WithRejection;
use garde::Validate;

use crate::{
    dto::{
        request::{
            case::{
                CreateFieldRequest, CreateFunctionalCaseRequest, DeleteFieldRequest,
                QueryFieldParam, UpdateFieldRequest,
            },
            file::{CreateModuleRequest, DeleteModuleRequest, QueryModuleParam},
            CaseQueryParam, CreateScriptRequest, DiagnoseRequest, IssueRelationRequest,
            ListQueryParam, QueryTemplateParam,
        },
        response::{
            case::FunctionalCaseResponse, CreateEntityResponse, CreateScriptResponse,
            DiagnoseResponse, FileModuleResponse, ListFunctionalCaseResponse, MessageResponse,
            RequirementInfoResponse, TemplateResponse,
        },
    },
    entity::case::Field,
    errors::{AppError, AppResponseError, AppResult},
    service::{self, case, file},
    state::AppState,
    utils::{
        claim::UserClaims,
        header::{extract_project_id, validate_project_id},
    },
};
use tracing::info;

#[utoipa::path(
    get,
    path = "/case/module/{project_id}",
    params(
        ("project_id", description = "path parameter"),
        ("module_id", description = "query parameter")
    ),
    responses(
        (status = 200, description = "Success get case module tree", body = [Vec<FileModuleResponse>]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "Not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn get(
    Extension(state): Extension<AppState>,
    Path(project_id): Path<i32>,
    Query(params): Query<QueryModuleParam>,
) -> AppResult<Json<Vec<FileModuleResponse>>> {
    info!("case module tree query param: {project_id:?}");
    match file::get_file_module(&state, &project_id, "CASE".into(), params).await {
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
    responses(
        (status = 200, description = "Success create case module", body = [MessageResponse]),
        (status = 400, description = "Invalid parameters", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn create_module(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    headers: HeaderMap,
    Json(request): Json<CreateModuleRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    info!("case module create with request: {request:?}");
    request.validate()?;
    validate_project_id(&headers, request.project_id)?;
    match file::create_file_module(&state, user.uid, "CASE".into(), &request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    delete,
    path = "/management/case/module",
    responses(
        (status = 200, description = "Success delete case module", body = [MessageResponse])
    ),
    security(("jwt" = []))
)]
pub async fn delete_module(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<DeleteModuleRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!(
        "controller layer delete case module with module_id: {}",
        request.id
    );
    match service::case::delete_by_module_id(&state, user.uid, request.id).await {
        Ok(_) => Ok(Json(MessageResponse::new("Success delete case module"))),
        Err(e) => Err(e),
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
    match case::template(&state, project_id).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/management/case/functional_case",
    request_body = CreateFunctionalCaseRequest,
    responses(
        (status = 200, description = "Success create functional case", body = [CreateEntityResponse]),
        (status = 400, description = "Invalid parameters", body = [AppResponseError]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 403, description = "Forbidden", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn create_functional_case(
    Extension(state): Extension<AppState>,
    user: UserClaims,
    Json(request): Json<CreateFunctionalCaseRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    info!("create functional case with request: {request:?}");
    request.validate()?;
    match case::create_functional_case(&state, user.uid, request).await {
        Ok(resp) => Ok(Json(CreateEntityResponse { id: resp })),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/case/functional-case/{case_id}",
    responses(
        (status = 200, description = "Success get functional-case", body = [FunctionalCaseResponse]),
        (status = 404, description = "Case not found", body = [AppResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_functional_case(
    Extension(state): Extension<AppState>,
    Path(case_id): Path<i32>,
    _user: UserClaims,
) -> AppResult<Json<FunctionalCaseResponse>> {
    info!("query functional case with path case_id: {case_id:?}");
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
) -> AppResult {
    match case::create_issue_relation(&state, user.uid, request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    post,
    path = "/management/case/field",
    responses(
        (status = 200, description = "Success create field", body = [CreateEntityResponse]),
    ),
)]
pub async fn create_field(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    user: UserClaims,
    Json(request): Json<CreateFieldRequest>,
) -> AppResult<Json<CreateEntityResponse>> {
    info!("case controller layer create field with {request:?}");
    request.validate()?;
    let project_id = extract_project_id(&headers)?;
    match case::create_field(&state, user.uid, project_id, request).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    put,
    path = "/management/case/field",
    responses(
        (status = 200, description = "Success update field", body = [MessageResponse]),
        (status = 400, description = "Invalid parameters", body = [AppResponseError])
    ),
    security(("jwt" =[]))
)]
pub async fn update_field(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    user: UserClaims,
    Json(request): Json<UpdateFieldRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("case controller layer update field with {request:?}");
    request.validate()?;
    let project_id = extract_project_id(&headers)?;
    match case::update_field(&state, user.uid, project_id, request).await {
        Ok(_) => Ok(Json(MessageResponse::new("Success update"))),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    delete,
    path = "/management/case/field",
    responses(
        (status = 200, description = "Success delete field", body = [MessageResponse])
    ),
    security(("jwt" = []))
)]
pub async fn delete_field(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    user: UserClaims,
    Json(request): Json<DeleteFieldRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("case controller layer delete field with {request:?}");
    let project_id = extract_project_id(&headers)?;
    match case::delete_field(&state, user.uid, project_id, request).await {
        Ok(_) => Ok(Json(MessageResponse::new("Success delete field"))),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/field/{project_id}",
    responses(
        (status = 200, description = "Success get field list", body = [Vec<Field>])
    ),
    security(("jwt" = []))
)]
pub async fn get_field_list(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path(project_id): Path<i32>,
    Query(params): Query<QueryFieldParam>,
) -> AppResult<Json<Vec<Field>>> {
    info!("controller layer query field list with project_id: {project_id:?}");
    validate_project_id(&headers, project_id)?;
    match case::get_field_list(&state, project_id, params).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/case/info/requirement/{project_id}",
    responses(
        (status = 200, description = "Get related pr info"),
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
    path = "/management/case/functional-case",
    params(ListQueryParam),
    responses(
        (status = 200, description = "Get case list"),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case list not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn get_functional_case_list(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Query(param): Query<ListQueryParam>,
) -> AppResult<Json<ListFunctionalCaseResponse>> {
    info!("controller layer query case list with param: {param:?}");
    let project_id = extract_project_id(&headers)?;
    match case::get_functional_case_list(&state, &project_id, &param).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => Err(e),
    }
}

#[utoipa::path(
    get,
    path = "/management/case/count/:project_id",
    params(CaseQueryParam),
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
    Query(param): Query<CaseQueryParam>,
) -> AppResult<Json<HashMap<String, i64>>> {
    info!("controller layer case count group by module in project: {project_id:?}");
    match case::count(&state, &project_id, &param).await {
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
