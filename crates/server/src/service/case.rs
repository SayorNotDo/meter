use std::collections::HashMap;

use crate::constant::PAGE_DECODE_KEY;
use crate::dao::case::CaseDao;
use crate::dao::element::ElementDao;
use crate::dao::entity::CustomField;
use crate::dao::file::FileDao;
use crate::dto::request::{CaseQueryParam, CreateScriptRequest, ListQueryParam};
use crate::dto::response::{
    CaseDetailResponse, CreateScriptResponse, ListCaseResponse, RequirementInfoResponse,
};
use crate::dto::{request::QueryTemplateParam, response::TemplateResponse};
use crate::errors::AppResult;
use crate::service::engine;
use crate::service::token::generate_page_token;
use crate::state::AppState;
use crate::utils::claim::PageClaims;
use tracing::info;
use uuid::Uuid;

pub async fn template(
    state: &AppState,
    project_id: &i32,
    param: &QueryTemplateParam,
) -> AppResult<TemplateResponse> {
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    /* Template */
    let template = case_dao.get_template(project_id, param.is_default).await?;
    /* Related Custom Fields */
    Ok(TemplateResponse {
        id: template.id,
        name: template.name,
        internal: template.internal,
        description: template.description,
        created_by: template.created_by,
        created_at: template.created_at,
        updated_at: template.updated_at,
        custom_fields: template.custom_fields,
    })
}

pub async fn field(
    state: &AppState,
    project_id: &i32,
    param: &QueryTemplateParam,
) -> AppResult<Vec<CustomField>> {
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    /* Fields with options */
    let fields = case_dao.get_fields(project_id, param.is_default).await?;
    Ok(fields)
}

pub async fn info(_state: &AppState, _project_id: &i32) -> AppResult<RequirementInfoResponse> {
    Ok(RequirementInfoResponse {})
}

pub async fn list(
    state: &AppState,
    project_id: &i32,
    param: &ListQueryParam,
) -> AppResult<ListCaseResponse> {
    info!("service layer for list with path_param: {project_id:?}, query_param: {param:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    /* processing page_token if exist else  */
    let (page_size, page_num) = match &param.page_token {
        Some(token) => {
            let page_claims = PageClaims::decode(token.as_str(), &PAGE_DECODE_KEY)?.claims;
            (page_claims.page_size, page_claims.page_num)
        }
        None => {
            let page_size = param.page_size.unwrap_or(10);
            (page_size, 0)
        }
    };
    let module_id = if let Some(id) = param.module_id {
        vec![id]
    } else {
        /* get root module_id while related query param is null */
        let file_dao = FileDao::new(&client);
        file_dao
            .get_root_module_id(project_id, "CASE".into())
            .await?
    };
    let offset = page_num * page_size;
    let next_page_token = generate_page_token(page_size, page_num + 1)?;
    let list = case_dao
        .get_case_list(project_id, &module_id, &page_size, &offset)
        .await?;
    Ok(ListCaseResponse {
        next_page_token,
        list,
    })
}

pub async fn count(
    state: &AppState,
    project_id: &i32,
    param: &CaseQueryParam,
) -> AppResult<HashMap<String, i64>> {
    info!("service layer for case count with project_id: {project_id:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let is_deleted = if let Some(is_deleted) = param.is_deleted {
        is_deleted
    } else {
        false
    };
    let hmap = case_dao.count(project_id, &is_deleted).await?;
    Ok(hmap)
}

pub async fn detail(state: &AppState, case_id: &i32) -> AppResult<CaseDetailResponse> {
    info!("service layer for case detail with case id: {case_id:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let detail = case_dao.detail(case_id).await?;
    let tags: Vec<String> = detail
        .tags
        .split(",")
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    Ok(CaseDetailResponse {
        id: detail.id,
        name: detail.name,
        project_id: detail.id,
        template_id: detail.template_id,
        status: detail.status,
        tags,
        module_name: detail.module_name,
        attach_info: detail.attach_info,
        created_at: detail.created_at,
        created_by: detail.created_by,
        custom_fields: detail.custom_fields,
    })
}

pub async fn gen_script(
    state: &AppState,
    uid: Uuid,
    request: CreateScriptRequest,
) -> AppResult<CreateScriptResponse> {
    info!("service layer generate script with request: {request:?}");
    /* construct DriveData with request parameters */
    let mut client = state.pool.get().await?;
    let element_dao = ElementDao::new(&mut client);
    let pre_processors = element_dao.get_elements(request.pre_processors).await?;

    /* generate script with engine service */
    let data = engine::DriveData {
        name: request.name,
        environment: request.environment.clone(),
        description: "".into(),
        pre_processors: vec![],
        steps: vec![],
        after_processors: vec![],
    };
    let mut script = engine::generator(data).await?;

    /* insert script record into database */
    let case_dao = CaseDao::new(&client);
    let related_case = case_dao.detail(&request.case_id).await?;
    let path = script.path.clone();
    script.case_id = related_case.id;
    script.environment = request.environment;
    script.created_by = uid;
    let script_id: i32 = case_dao.insert_script(script).await?;

    /* binding relationship for element used in script */

    Ok(CreateScriptResponse {
        id: script_id,
        path,
    })
}
