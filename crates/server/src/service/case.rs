use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
use tokio::try_join;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    constant::{DOCTOR_SCRIPT_PATH, PAGE_DECODE_KEY},
    dao::{case::CaseDao, element::ElementDao, entity::Step, file::FileDao},
    dto::{
        request::{
            case::{
                CreateFieldRequest, CreateFunctionalCaseRequest, DeleteFieldRequest,
                QueryFieldParam, UpdateFieldRequest, UpdateFunctionalCaseRequest,
            },
            CaseQueryParam, CreateScriptRequest, DiagnoseRequest, IssueRelationRequest,
            ListQueryParam,
        },
        response::{
            case::{FunctionalCaseResponse, GetTemplateResponse, ListFunctionalCaseResponse},
            CreateEntityResponse, CreateScriptResponse, DiagnoseResponse, RequirementInfoResponse,
        },
    },
    entity::case::{Field, FieldType, FieldValue, FunctionalCase},
    errors::{AppError, AppResult, Resource, ResourceType},
    service::{
        engine::{self, StepInfo},
        token::generate_page_token,
    },
    state::AppState,
    utils::claim::PageClaims,
};

pub async fn template(state: &AppState, project_id: i32) -> AppResult<GetTemplateResponse> {
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
    /* Template */
    let template = case_dao.get_template_project_id(project_id).await?;
    Ok(GetTemplateResponse {
        id: template.id,
        name: template.name,
        internal: template.internal,
        description: template.description,
        created_by: template.created_by,
        created_at: template.created_at,
        updated_at: template.updated_at,
        fields: template.fields,
    })
}

pub async fn create_field(
    state: &AppState,
    uid: Uuid,
    project_id: i32,
    request: CreateFieldRequest,
) -> AppResult<CreateEntityResponse> {
    info!("case service layer create field with {request:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let field_type = FieldType::from_str(&request.field_type);
    let field = Field::new(
        &request.name,
        &request.field_type,
        request.remark,
        project_id,
    );
    let field_id = case_dao.create_field(field, uid).await?;
    /* TODO: FieldOption Relations Insert */
    match field_type {
        FieldType::Input => {
            info!("fieldType `INPUT` no need to insert field option...")
        }
        FieldType::Select => {
            if let Some(options) = request.options {
                for option in options.into_iter() {
                    case_dao.insert_field_option(field_id, option, uid).await?;
                }
            } else {
                return Err(AppError::BadRequestError(
                    "Field `options` required".to_string(),
                ));
            }
        }
        FieldType::Unknown => {
            return Err(AppError::BadRequestError("Unknown fieldType".to_string()))
        }
    }
    transaction.commit().await?;
    Ok(CreateEntityResponse { id: field_id })
}

pub async fn update_field(
    state: &AppState,
    uid: Uuid,
    project_id: i32,
    request: UpdateFieldRequest,
) -> AppResult {
    info!("case service layer update field with {request:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let change_type = FieldType::from_str(&request.field_type);
    let mut field = case_dao.get_field_by_id(request.id).await?;
    if project_id != field.project_id {
        return Err(AppError::ForbiddenError("Access denied".to_string()));
    };
    field.name = request.name;
    field.field_type = request.field_type;
    field.remark = request.remark;
    /* update related FieldOption */
    match change_type {
        FieldType::Input => {
            case_dao
                .soft_delete_field_option_by_field_id(field.id, uid)
                .await?
        }
        FieldType::Select => {
            if let Some(options) = request.options {
                for option in options.into_iter() {
                    match case_dao.get_field_option_by_id(option.id).await {
                        Ok(mut o) => {
                            o.value = option.value;
                            o.position = option.position;
                            case_dao.update_field_option(o, uid).await?;
                        }
                        Err(AppError::NotFoundError(_)) => {
                            let _ = case_dao.insert_field_option(field.id, option, uid).await?;
                        }
                        Err(e) => return Err(e),
                    }
                }
            } else {
                return Err(AppError::BadRequestError(
                    "Field `oprtions` required".to_string(),
                ));
            }
        }
        FieldType::Unknown => {
            return Err(AppError::BadRequestError("Unknown field type".to_string()))
        }
    };
    case_dao.update_field(field, uid).await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn delete_field(
    state: &AppState,
    uid: Uuid,
    project_id: i32,
    request: DeleteFieldRequest,
) -> AppResult {
    info!("case service layer delete field with {request:?}, project_id: {project_id}, deleted_by: {uid}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let field = case_dao.get_field_by_id(request.id).await?;
    if field.project_id != project_id {
        return Err(AppError::ForbiddenError("Access denied".to_string()));
    }
    case_dao.soft_delete_field(request.id, uid).await?;
    if let FieldType::Select = FieldType::from_str(&field.field_type) {
        for option in field.options {
            case_dao.soft_delete_field_option(option.id, uid).await?;
        }
    };
    transaction.commit().await?;
    Ok(())
}

pub async fn get_field_list(
    state: &AppState,
    project_id: i32,
    params: QueryFieldParam,
) -> AppResult<Vec<Field>> {
    let mut client = state.pool.get().await?;
    let case_dao = CaseDao::new(&mut client);
    /* Fields with options */
    let fields = match params.field_id {
        Some(field_id) => {
            let field = case_dao.get_field_by_id(field_id).await?;
            vec![field]
        }
        None => case_dao.get_fields(project_id).await?,
    };
    Ok(fields)
}

pub async fn create_functional_case(
    state: &AppState,
    uid: Uuid,
    request: CreateFunctionalCaseRequest,
) -> AppResult<i32> {
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let file_dao = FileDao::new(&transaction);
    let module = file_dao.get_module_by_id(request.module_id).await?;
    /* insert into functional_cases */
    let case = FunctionalCase::new(&request.name, module, request.template_id, request.tags);
    /* check template is exist or not, otherwise return not found err */
    let template = case_dao.get_template_by_id(case.template_id).await?;

    let mut template_required_field_ids: HashSet<_> = template
        .fields
        .iter()
        .filter(|f| f.required)
        .map(|f| f.id)
        .collect();
    warn!("=================>>> required_field_ids: {template_required_field_ids:?}");

    let template_optional_field_ids: HashSet<_> = template
        .fields
        .iter()
        .filter(|f| !f.required)
        .map(|f| f.id)
        .collect();

    let allowed_field_ids: HashSet<_> = template_required_field_ids
        .union(&template_optional_field_ids)
        .cloned()
        .collect();

    let case_id = case_dao.insert_functional_case(case, uid).await?;
    /* bind relationship between case with custom_field through table: [functional_case_field_relation] */
    for item in request.fields.into_iter() {
        /* get fielld by field_id */
        if !allowed_field_ids.contains(&item.id) {
            return Err(AppError::BadRequestError(format!(
                "Field id `{}` not allowed",
                item.id
            )));
        }
        let field = case_dao.get_field_by_id(item.id).await?;
        let field_type = FieldType::from_str(&field.field_type);
        match (field_type, item.value) {
            (FieldType::Input, FieldValue::Input(value)) => {
                case_dao
                    .insert_case_field_relation(case_id, field.id, &value, uid)
                    .await?;
            }
            (FieldType::Select, FieldValue::Select(option)) => {
                let value = option.to_string();
                case_dao
                    .insert_case_field_relation(case_id, field.id, &value, uid)
                    .await?;
            }
            (FieldType::Unknown, _) => {
                return Err(AppError::BadRequestError("Unknow fieldType".to_string()))
            }
            (_, FieldValue::Input(_) | FieldValue::Select(_)) => {
                return Err(AppError::BadRequestError(
                    "fieldType mismatch with fieldValue".to_string(),
                ));
            }
        }
        template_required_field_ids.remove(&item.id);
    }
    if !template_required_field_ids.is_empty() {
        return Err(AppError::BadRequestError(
            "Missing required field".to_string(),
        ));
    }

    transaction.commit().await?;
    Ok(case_id)
}

pub async fn update_functional_case(
    state: &AppState,
    project_id: i32,
    updated_by: Uuid,
    request: UpdateFunctionalCaseRequest,
) -> AppResult {
    info!("case service layer update functional case with request: {request:?}, updated_by: {updated_by}, project_id: {project_id}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let file_dao = FileDao::new(&transaction);
    let module = file_dao.get_module_by_id(request.module_id).await?;
    let mut case = case_dao.get_functional_case_by_id(request.case_id).await?;
    info!("===>> : {case:?}");
    /* Setter */
    case.name = request.name;
    case.module = module;
    /* Update case */
    match case_dao.get_functional_case_by_name(case.name).await {
        Ok(r) => {
            if r.id != case.id {
                Err(AppError::ResourceExistsError(Resource {
                    details: vec![],
                    resource_type: ResourceType::Case,
                }))
            } else {
                Ok(())
            }
        }
        Err(AppError::NotFoundError { .. }) => {
            info!("case modify name haven't been used");
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_by_module_id(state: &AppState, uid: Uuid, module_id: i32) -> AppResult {
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let file_dao = FileDao::new(&transaction);

    file_dao.soft_delete_by_id(uid, module_id).await?;
    transaction.commit().await?;

    Ok(())
}

pub async fn get_functional_case(
    state: &AppState,
    case_id: i32,
) -> AppResult<FunctionalCaseResponse> {
    info!("service layer get functional case with case_id {case_id:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let case = case_dao.get_functional_case_by_id(case_id).await?;
    let fields = case_dao.get_fields_by_case_id(case.id).await?;
    Ok(FunctionalCaseResponse {
        id: case.id,
        name: case.name,
        tags: case.tags,
        template_id: case.template_id,
        module: case.module,
        status: case.status.to_string(),
        created_at: case.created_at,
        created_by: case.created_by,
        updated_at: case.updated_at,
        updated_by: case.updated_by,
        attach_info: case.attach_info,
        fields,
    })
}

pub async fn create_issue_relation(
    state: &AppState,
    uid: Uuid,
    request: IssueRelationRequest,
) -> AppResult {
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

pub async fn get_functional_case_list(
    state: &AppState,
    project_id: &i32,
    param: &ListQueryParam,
) -> AppResult<ListFunctionalCaseResponse> {
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
    let module_ids = if let Some(id) = param.module_id {
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
    let functional_case_list = case_dao
        .get_functional_case_list(module_ids, page_size, offset)
        .await?;
    let mut list: Vec<FunctionalCaseResponse> = Vec::new();
    for case in functional_case_list.into_iter() {
        let fields = case_dao.get_fields_by_case_id(case.id).await?;
        list.push(FunctionalCaseResponse {
            id: case.id,
            name: case.name,
            template_id: case.template_id,
            module: case.module,
            created_at: case.created_at,
            created_by: case.created_by,
            updated_at: case.updated_at,
            updated_by: case.updated_by,
            attach_info: case.attach_info,
            fields,
            tags: case.tags,
            status: case.status.to_string(),
        })
    }

    transaction.commit().await?;
    Ok(ListFunctionalCaseResponse {
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
    let hmap = match param.deleted {
        Some(true) => case_dao.count_deleted_case(project_id).await?,
        _ => case_dao.count(project_id).await?,
    };
    Ok(hmap)
}

pub async fn detail(state: &AppState, case_id: i32) -> AppResult<FunctionalCaseResponse> {
    info!("service layer for case detail with case id: {case_id:?}");
    let client = state.pool.get().await?;
    let case_dao = CaseDao::new(&client);
    let case = case_dao.get_functional_case_by_id(case_id).await?;
    // let tags: Vec<String> = if let Some(d) = case.tags {
    //     d.split(",")
    //         .into_iter()
    //         .map(|s| s.to_string())
    //         .collect::<Vec<_>>()
    // } else {
    //     Vec::new()
    // };
    let fields = case_dao.get_fields_by_case_id(case_id).await?;
    Ok(FunctionalCaseResponse {
        id: case.id,
        name: case.name,
        template_id: case.template_id,
        status: case.status.to_string(),
        tags: case.tags,
        module: case.module,
        attach_info: case.attach_info,
        updated_at: case.updated_at,
        updated_by: case.updated_by,
        created_at: case.created_at,
        created_by: case.created_by,
        fields,
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
    let related_case = case_dao.get_functional_case_by_id(request.case_id).await?;
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
        case_dao.insert_script_element_relation(&script_id, "STEP".to_string(), &request.steps),
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

#[allow(dead_code)]
pub async fn exec_case(_state: &AppState) -> AppResult {
    Ok(())
}
