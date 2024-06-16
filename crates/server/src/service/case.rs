use crate::dao::case::CaseDao;
use crate::dto::{request::QueryTemplateParam, response::TemplateResponse};
use crate::errors::AppResult;
use crate::state::AppState;

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
        custom_fields: template.custom_fields
    })
}
