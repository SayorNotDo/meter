use std::collections::HashMap;
use tracing::info;
use crate::dto::response::{FileModuleResponse};
use crate::errors::{AppError, AppResult, Resource, ResourceType};
use crate::state::AppState;
use crate::dao;

pub async fn module_tree(state: &AppState, project_id: &i32) -> AppResult<Vec<FileModuleResponse>> {
    let mut file_module_tree = vec![];
    let client = state.pool.get().await?;
    let file_dao = dao::file::FileDao::new(&client);
    let file_modules = file_dao.get_file_modules(project_id).await?;
    /* 创建HashMap 用于快速查找父节点 */
    let mut module_map: HashMap<i32, FileModuleResponse> = HashMap::new();

    /* 初始化节点 */
    for item in file_modules.iter() {
        module_map.insert(item.id, FileModuleResponse {
            id: item.id,
            name: item.name.clone(),
            path: "".to_string(),
            module_type: item.module_type.clone(),
            parent_id: item.parent_id,
            children: Vec::new(),
        });
    }

    /* 构建树结构 */
    for item in file_modules.iter() {
        if item.parent_id.is_none() {
            if let Some(mut root) = module_map.remove(&item.id) {
                build_tree(&mut root, &mut module_map);
                file_module_tree.push(root);
            }
        }
    }

    for root in file_module_tree.iter_mut() {
        update_path(root, "".to_string());
    }

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