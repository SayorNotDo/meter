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
    })
}

pub async fn list(state: &AppState, uid: Uuid) -> AppResult<ProjectListResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(client);
    let projects = project_dao.find_projects_by_uid(uid).await?;
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