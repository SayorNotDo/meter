use crate::constant::PAGE_DECODE_KEY;
use crate::dao::case::CaseDao;
use crate::dao::entity::CustomField;
use crate::dao::file::FileDao;
use crate::dto::request::ListQueryParam;
use crate::dto::response::{ListCaseResponse, RequirementInfoResponse};
use crate::dto::{request::QueryTemplateParam, response::TemplateResponse};
use crate::errors::AppResult;
use crate::service::token::generate_page_token;
use crate::state::AppState;
use crate::utils::claim::PageClaims;
use tracing::info;

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
