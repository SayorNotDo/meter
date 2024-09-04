use crate::constant::DOCTOR_SCRIPT_PATH;
use std::{collections::HashMap, path::Path};
use tokio::try_join;
use tracing::info;
use uuid::Uuid;

use crate::{
    constant::PAGE_DECODE_KEY,
    dto::{
        request::{
            CaseQueryParam, CreateFunctionalCaseRequest, CreateScriptRequest, DiagnoseRequest,
            ListQueryParam, QueryTemplateParam,
        },
        response::{
            CaseDetailResponse, CreateScriptResponse, DiagnoseResponse, ListCaseResponse,
            RequirementInfoResponse, TemplateResponse,
        },
    },
    errors::AppResult,
    service::{
        engine::{self, StepInfo},
        token::generate_page_token,
    },
    state::AppState,
    utils::claim::PageClaims,
};
use crate::{
    dao::{
        case::CaseDao,
        element::ElementDao,
        entity::{CustomField, FunctionalCase, Step},
        file::FileDao,
    },
    dto::request::IssueRelationRequest,
};

pub async fn template(
    state: &AppState,
    project_id: &i32,
    param: &QueryTemplateParam,
) -> AppResult<TemplateResponse> {
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
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
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
    /* Fields with options */
    let fields = case_dao.get_fields(project_id, param.is_default).await?;
    Ok(fields)
}

pub async fn create_functional_case(
    state: &AppState,
    uid: Uuid,
    request: CreateFunctionalCaseRequest,
) -> AppResult<()> {
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    /* insert into functional_cases */
    let case = FunctionalCase::new(
        request.name.as_str(),
        request.module_id,
        request.template_id,
        request.tags,
        uid,
    );
    let case_id = case_dao.insert_functional_case(case).await?;
    /* bind relationship between case with custom_field through table: [functional_case_custom_field]*/
    case_dao
        .insert_case_field_relation(case_id, request.custom_fields)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn get_functional_case(state: &AppState, case_id: i32) -> AppResult<CaseDetailResponse> {
    info!("service layer get functional case with case_id {case_id:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let functional_case = case_dao.detail(&case_id).await?;
    let tags: Vec<String> = if let Some(d) = functional_case.tags {
        d.split(",")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    Ok(CaseDetailResponse {
        id: functional_case.id,
        name: functional_case.name,
        tags,
        template_id: functional_case.template_id,
        module_name: functional_case.module_name,
        status: functional_case.status,
        created_at: functional_case.created_at,
        created_by: functional_case.created_by,
        attach_info: functional_case.attach_info,
        custom_fields: functional_case.custom_fields,
    })
}

pub async fn create_issue_relation(
    state: &AppState,
    uid: Uuid,
    request: IssueRelationRequest,
) -> AppResult<()> {
    info!("service layer create issue relation with request: {request:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    for item in request.issues.iter() {
        case_dao
            .insert_case_issue_relation(&request.case_id, item, &uid)
            .await?;
    }
    transaction.commit().await?;
    Ok(())
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
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
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
        let file_dao = FileDao::new(&transaction);
        file_dao
            .get_root_module_id(project_id, "CASE".into())
            .await?
    };
    let offset = page_num * page_size;
    let next_page_token = generate_page_token(page_size, page_num + 1)?;
    let case_dao = CaseDao::new(&transaction);
    let list = case_dao
        .get_case_list(&module_id, &page_size, &offset)
        .await?;
    transaction.commit().await?;
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
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
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
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
    let detail = case_dao.detail(case_id).await?;
    let tags: Vec<String> = if let Some(d) = detail.tags {
        d.split(",")
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    Ok(CaseDetailResponse {
        id: detail.id,
        name: detail.name,
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

async fn get_step_list<T>(dao: &ElementDao<'_, T>, req: &Vec<Step>) -> AppResult<Vec<StepInfo>>
where
    T: db::GenericClient,
{
    info!("get step list with params: {req:?}");
    let mut info_list = Vec::new();
    for item in req.iter() {
        if let Ok(info) = dao.get_element(item.element_id, item.option_id).await {
            info_list.push(StepInfo {
                position: item.position,
                action: info.action,
                selector: info.selector,
                attach_info: item.attach_info.clone(),
            })
        }
    }
    Ok(info_list)
}

pub async fn gen_script(
    state: &AppState,
    uid: Uuid,
    request: CreateScriptRequest,
) -> AppResult<CreateScriptResponse> {
    info!("service layer generate script with request: {request:?}");
    /* construct DriveData with request parameters */
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let element_dao = ElementDao::new(&transaction);
    let (pre_processors, steps, after_processors) = try_join!(
        get_step_list(&element_dao, &request.pre_processors),
        get_step_list(&element_dao, &request.steps),
        get_step_list(&element_dao, &request.after_processors)
    )?;

    /* generate script with engine service */
    let data = engine::DriveData {
        name: request.name,
        environment: request.environment.clone(),
        description: "".into(),
        pre_processors,
        steps,
        after_processors,
    };
    let mut script = engine::generator(data).await?;

    /* insert script record into database */
    let case_dao = CaseDao::new(&transaction);
    let related_case = case_dao.detail(&request.case_id).await?;
    let path = script.path.clone();
    script.case_id = related_case.id;
    script.environment = request.environment;
    script.created_by = uid;
    let script_id: i32 = case_dao.insert_script(script).await?;

    /* binding relationship for element used in script */
    try_join!(
        case_dao.insert_script_element_relation(
            &script_id,
            "PRE_PROCESSOR".to_string(),
            &request.pre_processors,
        ),
        case_dao.insert_script_element_relation(&script_id, "STEP".to_string(), &request.steps,),
        case_dao.insert_script_element_relation(
            &script_id,
            "AFTER_PROCESSOR".to_string(),
            &request.after_processors,
        )
    )?;
    transaction.commit().await?;
    Ok(CreateScriptResponse {
        id: script_id,
        path,
    })
}

pub async fn env_diagnose(
    state: &AppState,
    request: DiagnoseRequest,
) -> AppResult<DiagnoseResponse> {
    /* script path */
    let script_path_str = format!("{}/{}", DOCTOR_SCRIPT_PATH, &request.script_name);

    /* get specific machine from db */
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let machine = case_dao.get_machine(&request.machine_id).await?;

    let resp = engine::doctor_script(machine, Path::new(&script_path_str)).await?;
    Ok(DiagnoseResponse { msg: resp })
}

pub async fn exec_case(state: &AppState) -> AppResult<()> {
    Ok(())
}
