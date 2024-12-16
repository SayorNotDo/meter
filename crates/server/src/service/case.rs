use std::{collections::HashMap, path::Path};

use tokio::try_join;
use tracing::info;
use uuid::Uuid;

use crate::{
    constant::{case::CASE_NUM, DOCTOR_SCRIPT_PATH},
    dao::{case::CaseDao, element::ElementDao, entity::Step, file::FileDao},
    dto::{
        request::{
            case::{
                CreateFieldRequest, CreateFunctionalCaseRequest, DeleteFieldRequest,
                QueryFieldParam, UpdateFieldRequest, UpdateFunctionalCaseRequest,
            },
            CaseQueryParam, CreateScriptRequest, DeleteEntityRequest, DiagnoseRequest,
            IssueRelationRequest, ListQueryParam,
        },
        response::{
            case::{FunctionalCaseResponse, GetTemplateResponse, ListFunctionalCaseResponse},
            CreateEntityResponse, CreateScriptResponse, DiagnoseResponse, RequirementInfoResponse,
        },
    },
    entity::case::{CaseResult, Field, FieldType, FieldValue, FunctionalCase, TemplateField},
    errors::{AppError, AppResult, Resource, ResourceType},
    service::{
        engine::{self, StepInfo},
        token::{generate_page_token, parse_page_token},
    },
    state::AppState,
    utils::{claim::PageClaims, parse_ids},
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
    if let Some(id) = params.field_id {
        return Ok(vec![case_dao.get_field_by_id(id).await?]);
    }
    case_dao.get_fields(project_id).await
}

fn field_classify(field_set: Vec<TemplateField>) -> AppResult<(Vec<i32>, Vec<i32>)> {
    let mut template_required_field_ids = Vec::new();
    let mut allowed_field_ids = Vec::new();

    for field in field_set {
        if field.required {
            template_required_field_ids.push(field.id);
        }
        allowed_field_ids.push(field.id);
    }
    Ok((template_required_field_ids, allowed_field_ids))
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
    /* check template exist or not, otherwise return not found err */
    let template = case_dao.get_template_by_id(case.template_id).await?;

    let (template_required_field_ids, allowed_field_ids) = field_classify(template.fields)?;
    let request_field_ids: Vec<i32> = request
        .fields
        .iter()
        .filter(|f| f.required)
        .map(|f| f.id)
        .collect();
    if request_field_ids != template_required_field_ids {
        return Err(AppError::BadRequestError(
            "Missing require field".to_string(),
        ));
    }
    let case_id = case_dao.insert_functional_case(case, uid).await?;
    /* bind relationship between case with custom_field through table: [functional_case_field_relation] */
    for item in request.fields.into_iter() {
        /* get field by field_id */
        if !allowed_field_ids.contains(&item.id) {
            return Err(AppError::BadRequestError(format!(
                "Field id `{}` not allowed",
                item.id
            )));
        }
        let field = case_dao.get_field_by_id(item.id).await?;
        /* TODO: Check whether field is unique or not while field is unique_required is true */
        if &field.name == CASE_NUM {
            case_dao
                .check_unique_by_field_id_and_value(&field.id, &item.value)
                .await?
        }
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
                return Err(AppError::BadRequestError("Unknown fieldType".to_string()))
            }
            (_, FieldValue::Input(_) | FieldValue::Select(_)) => {
                return Err(AppError::BadRequestError(
                    "fieldType mismatch with fieldValue".to_string(),
                ));
            }
        }
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
    let mut case = case_dao.get_functional_case_by_id(request.id).await?;
    /* Setter */
    case.name = request.name;
    case.module = module;
    /* Update case */
    match case_dao
        .get_functional_case_by_name(case.name.clone())
        .await
    {
        Ok(r) if r.id != case.id => Err(AppError::ResourceExistsError(Resource {
            details: vec![],
            resource_type: ResourceType::Case,
        })),
        Ok(_) | Err(AppError::NotFoundError { .. }) => {
            case_dao.update_functional_case(&case, updated_by).await?;
            /* functional_case_field_realtion update */
            for item in request.fields.into_iter() {
                let field = case_dao
                    .get_case_field_by_case_id_and_field_id(item.id, case.id)
                    .await?;
                match (field.field_type, item.value) {
                    (FieldType::Input, FieldValue::Input(value)) => {
                        if field.field_name == CASE_NUM {
                            case_dao
                                .check_unique_by_field_id_and_value(
                                    &field.id,
                                    &FieldValue::Input(value.clone()),
                                )
                                .await?
                        }
                        case_dao
                            .update_case_field_relation(field.id, &value, updated_by)
                            .await?;
                    }
                    (FieldType::Select, FieldValue::Select(option)) => {
                        let value = option.to_string();
                        case_dao
                            .update_case_field_relation(field.id, &value, updated_by)
                            .await?;
                    }
                    (_, FieldValue::Input(_) | FieldValue::Select(_)) => {
                        return Err(AppError::BadRequestError(
                            "mismatch field_value".to_string(),
                        ));
                    }
                }
            }
            transaction.commit().await?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub async fn delete_functional_case(
    state: &AppState,
    project_id: i32,
    deleted_by: Uuid,
    request: DeleteEntityRequest,
) -> AppResult {
    info!("case service layer delete functional case with request: {request:?}, deleted_by: {deleted_by}, project_id: {project_id}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let case = case_dao.get_functional_case_by_id(request.id).await?;
    case_dao
        .soft_delete_functional_case(case.id, deleted_by)
        .await?;
    /*TODO: delete related-resource*/
    case_dao
        .soft_delete_case_field_relation_by_case_id(case.id, deleted_by)
        .await?;
    transaction.commit().await?;
    Ok(())
}

pub async fn delete_by_module_id(state: &AppState, uid: Uuid, module_id: i32) -> AppResult {
    info!("case service layer delete case module with {module_id}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let file_dao = FileDao::new(&transaction);
    let case_dao = CaseDao::new(&transaction);
    let module = file_dao.get_module_by_id(module_id).await?;
    file_dao.soft_delete_by_id(uid, module.id).await?;
    case_dao
        .soft_delete_functional_case_by_module_id(module.id, uid)
        .await?;
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
    let last_execute_result = case_dao
        .get_last_execute_record_by_case_id(case.id)
        .await
        .map_or(CaseResult::UnExecuted, |f| f.result);
    Ok(FunctionalCaseResponse {
        id: case.id,
        name: case.name,
        tags: case.tags,
        template_id: case.template_id,
        module: case.module,
        status: case.status,
        edit_type: case.edit_type,
        created_at: case.created_at,
        created_by: case.created_by,
        updated_at: case.updated_at,
        updated_by: case.updated_by,
        attach_info: case.attach_info,
        last_execute_result,
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
    param: ListQueryParam,
) -> AppResult<ListFunctionalCaseResponse> {
    info!("service layer for list with project_id: {project_id:?}, query_param: {param:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let case_dao = CaseDao::new(&transaction);
    let page_claims = match param.page_token {
        Some(page_token) => parse_page_token(page_token)?,
        None => {
            let page_size = param.page_size.unwrap_or(10).clamp(1, 100);
            let page_num = param.page_num.unwrap_or(1).max(1);
            let offset = (page_num - 1) * page_size;
            let last_item_id = if offset > 0 {
                case_dao.get_query_cursor(offset).await?
            } else {
                0
            };
            let module_ids = match param.module_ids {
                Some(ids) => parse_ids(&ids)?,
                None => {
                    let file_dao = FileDao::new(&transaction);
                    file_dao
                        .get_all_module_id(project_id, "CASE".into())
                        .await?
                }
            };
            PageClaims::new(page_size, page_num, last_item_id, module_ids)
        }
    };
    let total = case_dao
        .count_case_by_module_ids(&page_claims.module_ids)
        .await?;
    let deleted = param.deleted.unwrap_or(false);
    let functional_case_list = case_dao
        .get_functional_case_list(
            &page_claims.module_ids,
            page_claims.last_item_id,
            page_claims.page_size,
            deleted,
        )
        .await?;
    let next_cursor = match functional_case_list.last() {
        Some(l) => l.id,
        None => 0,
    };
    let next_page_token = generate_page_token(
        page_claims.page_size,
        page_claims.page_num + 1,
        next_cursor,
        page_claims.module_ids,
    )?;
    let mut list: Vec<FunctionalCaseResponse> = Vec::new();
    for case in functional_case_list.into_iter() {
        let fields = case_dao.get_fields_by_case_id(case.id).await?;
        let last_execute_result = case_dao
            .get_last_execute_record_by_case_id(case.id)
            .await
            .map_or(CaseResult::UnExecuted, |f| f.result);
        list.push(FunctionalCaseResponse {
            id: case.id,
            name: case.name,
            template_id: case.template_id,
            module: case.module,
            created_at: case.created_at,
            created_by: case.created_by,
            edit_type: case.edit_type,
            updated_at: case.updated_at,
            updated_by: case.updated_by,
            attach_info: case.attach_info,
            last_execute_result,
            fields,
            tags: case.tags,
            status: case.status,
        })
    }
    transaction.commit().await?;
    Ok(ListFunctionalCaseResponse {
        next_page_token,
        list,
        total,
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
