use crate::dao::base::BaseDao;
use crate::dto::response::{MessageResponse, ProjectInfoResponse, ProjectListResponse};
use crate::errors::AppResult;
use uuid::Uuid;
use crate::state::AppState;
use crate::dao::project::*;


/* 获取项目信息 */
pub async fn info(state: &AppState, project_id: i32) -> AppResult<ProjectInfoResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(client);
    let project = project_dao.find_by_id(project_id).await?;
    Ok(ProjectInfoResponse {
        id: project.id,
        name: project.name,
        organization: project.organization,
        description: project.description,
        created_by: project.created_by,
        created_at: project.created_at,
        module_setting: project.module_setting,
        updated_at: project.updated_at
    })
}

pub async fn list(state: &AppState, uid: Uuid) -> AppResult<ProjectListResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(client);
    let ret = project_dao.find_projects_by_uid(uid).await?;
    let mut projects = Vec::new();
    for item in ret.iter() {
        let project = ProjectInfoResponse {
            id: item.id,
            name: item.name.clone(),
            organization: item.organization.clone(),
            description: item.description.clone(),
            created_by: item.created_by,
            created_at: item.created_at,
            module_setting: item.module_setting.clone(),
            updated_at: item.updated_at
        };
        projects.push(project);
    }
    Ok(ProjectListResponse {
        projects
    })
}

pub async fn permission(state: &AppState, project_id: i32, uid: Uuid) -> AppResult<MessageResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(client);
    project_dao.check_permission_by_uid(project_id, uid).await?;
    Ok(MessageResponse {
        message: "Check permission successfully".to_string(),
    })
}