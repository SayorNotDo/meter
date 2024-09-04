use tracing::info;

use crate::dao;
use crate::dto::response::FileModuleResponse;
use crate::errors::AppResult;
use crate::state::AppState;
use std::collections::HashMap;

#[derive(Debug)]
enum ModuleType {
    Case,
    Unknown,
}

impl ModuleType {
    fn from_str(module_type: &str) -> Self {
        match module_type {
            "CASE" => ModuleType::Case,
            _ => ModuleType::Unknown,
        }
    }
}

pub async fn file_module_tree(
    state: &AppState,
    project_id: &i32,
    module_type: &str,
) -> AppResult<Vec<FileModuleResponse>> {
    let mut file_module_tree = Vec::new();
    let mut client = state.pool.get().await?;
    let file_dao = dao::file::FileDao::new(&mut client);
    let file_modules = file_dao.get_file_modules(project_id, module_type).await?;
    /* 创建HashMap 用于快速查找父节点 */
    let mut module_map: HashMap<i32, FileModuleResponse> = HashMap::new();
    /* 初始化节点 */
    for item in file_modules.iter() {
        let item_type = ModuleType::from_str(&item.module_type);
        let count = match item_type {
            ModuleType::Case => {
                info!("get case count by module_id: {:?}", &item.id);
                let case_dao = dao::case::CaseDao::new(&mut client);
                let ret = case_dao.count_by_module_id(&item.id, &false).await?;
                ret
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
    /* 构建树结构 */
    for item in file_modules.iter() {
        if item.parent_id.is_none() {
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
