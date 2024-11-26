use crate::{
    dao,
    dto::{
        request::file::{CreateModuleRequest, QueryModuleParam},
        response::{CreateEntityResponse, FileModuleResponse},
    },
    entity::file::FileModule,
    errors::{AppError, AppResult},
    state::AppState,
};
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

#[derive(Debug)]
enum ModuleType {
    Case,
    Plan,
    Element,
    Unknown,
}

impl ModuleType {
    fn from_str(module_type: &str) -> Self {
        match module_type {
            "CASE" => ModuleType::Case,
            "PLAN" => ModuleType::Plan,
            "ELEMENT" => ModuleType::Element,
            _ => ModuleType::Unknown,
        }
    }
}

pub async fn create_file_module(
    state: &AppState,
    uid: Uuid,
    module_type: &str,
    request: &CreateModuleRequest,
) -> AppResult<CreateEntityResponse> {
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let file_dao = dao::file::FileDao::new(&transaction);
    let project_dao = dao::project::ProjectDao::new(&transaction);
    let descendant = if let Some(parent_id) = request.parent_id {
        file_dao
            .get_module_by_id(parent_id)
            .await
            .map_err(|e| AppError::BadRequestError(e.to_string()))?;
        file_dao
            .get_descendant_by_id(parent_id)
            .await
            .map_err(|e| AppError::BadRequestError(e.to_string()))?
    } else {
        file_dao
            .get_root_module_by_id(request.project_id, module_type)
            .await
            .map_err(|e| AppError::BadRequestError(e.to_string()))?
    };
    let position = if let Some(max_num) = descendant.into_iter().map(|item| item.position).max() {
        max_num + 1
    } else {
        0
    };

    project_dao
        .find_by_id(request.project_id)
        .await
        .map_err(|e| AppError::BadRequestError(e.to_string()))?;
    let file_module = FileModule {
        id: 0,
        name: request.name.clone(),
        position,
        module_type: module_type.into(),
        parent_id: request.parent_id,
    };
    let module_id = file_dao
        .insert_file_module(&uid, request.project_id, &file_module)
        .await?;
    transaction.commit().await?;
    Ok(CreateEntityResponse { id: module_id })
}

pub async fn get_file_module(
    state: &AppState,
    project_id: &i32,
    module_type: &str,
    params: QueryModuleParam,
) -> AppResult<Vec<FileModuleResponse>> {
    let mut file_module_tree = Vec::new();
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let file_dao = dao::file::FileDao::new(&transaction);
    let project_dao = dao::project::ProjectDao::new(&transaction);
    project_dao.find_by_id(project_id.clone()).await?;
    let file_modules: Vec<FileModule> = if let Some(module_id) = params.module_id {
        let module = file_dao.get_module_by_id(module_id).await?;
        vec![module]
    } else {
        file_dao.get_file_modules(project_id, module_type).await?
    };
    /* 创建HashMap 用于快速查找父节点 */
    let mut module_map: HashMap<i32, FileModuleResponse> = HashMap::new();
    /* 初始化节点 */
    for item in file_modules.iter() {
        let item_type = ModuleType::from_str(&item.module_type);
        let count = match item_type {
            ModuleType::Case => {
                let case_dao = dao::case::CaseDao::new(&transaction);
                case_dao.count_by_module_id(&item.id).await?
            }
            ModuleType::Plan => {
                let plan_dao = dao::plan::PlanDao::new(&transaction);
                plan_dao.count_by_module_id(&item.id, false).await?
            }
            ModuleType::Element => {
                let element_dao = dao::element::ElementDao::new(&transaction);
                element_dao.count_by_module_id(&item.id, false).await?
            }
            ModuleType::Unknown => {
                info!("unknown type");
                0
            }
        };
        module_map.insert(
            item.id,
            FileModuleResponse {
                id: item.id,
                name: item.name.clone(),
                path: "".to_string(),
                module_type: item.module_type.clone(),
                parent_id: item.parent_id,
                children: Vec::new(),
                count,
            },
        );
    }
    info!("original data for module_map: {module_map:?}");
    /* 构建树结构 */
    for item in file_modules.iter() {
        if item.parent_id.is_none() || Some(0) == item.parent_id {
            if let Some(mut root) = module_map.remove(&item.id) {
                build_tree(&mut root, &mut module_map);
                file_module_tree.push(root);
            }
        }
    }
    /* 更新节点path信息 */
    for root in file_module_tree.iter_mut() {
        update_path(root, "".to_string())
    }
    transaction.commit().await?;
    info!("finish construct module tree: {file_module_tree:?}");
    Ok(file_module_tree)
}

fn build_tree(node: &mut FileModuleResponse, module_map: &mut HashMap<i32, FileModuleResponse>) {
    let children_ids: Vec<i32> = module_map
        .values()
        .filter(|module| module.parent_id == Some(node.id))
        .map(|module| module.id)
        .collect();

    for id in children_ids {
        if let Some(mut child) = module_map.remove(&id) {
            build_tree(&mut child, module_map);
            node.children.push(child);
        }
    }
}

fn update_path(node: &mut FileModuleResponse, parent_path: String) {
    node.path = if parent_path.is_empty() {
        format!("/{}", node.name.clone())
    } else {
        format!("{}/{}", parent_path, node.name)
    };

    for child in node.children.iter_mut() {
        update_path(child, node.path.clone());
    }
}
