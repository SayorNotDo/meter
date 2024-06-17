use crate::dao::entity::ProjectMember;
use crate::dao::project::*;
use crate::dao::user::UserDao;
use crate::dto::response::{MessageResponse, ProjectInfoResponse};
use crate::errors::AppResult;
use crate::state::AppState;
use uuid::Uuid;

/* 获取项目信息 */
pub async fn info(state: &AppState, project_id: i32) -> AppResult<ProjectInfoResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(&client);
    let project = project_dao.find_by_id(project_id).await?;
    let user_dao = UserDao::new(&client);
    let admin_list = user_dao
        .find_by_role_and_project_id("admin", project_id)
        .await?;
    let module_list = match project.module_setting {
        Some(s) => serde_json::from_str(s.as_str()),
        None => Ok(vec![]),
    };
    Ok(ProjectInfoResponse {
        id: project.id,
        name: project.name,
        member_count: project.member_count,
        organization: project.organization,
        description: project.description,
        created_by: project.created_by,
        created_at: project.created_at,
        module_list: module_list?,
        creator_is_admin: true,
        updated_at: project.updated_at,
        updated_by: project.updated_by,
        deleted: project.deleted,
        deleted_at: project.deleted_at,
        deleted_by: project.deleted_by,
        admin_list,
    })
}

pub async fn list(
    state: &AppState,
    uid: Uuid,
    organization_id: i32,
) -> AppResult<Vec<ProjectInfo>> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(&client);
    let projects = project_dao
        .find_projects_by_uid(uid, organization_id)
        .await?;
    Ok(projects)
}

pub async fn permission(
    state: &AppState,
    project_id: i32,
    uid: Uuid,
) -> AppResult<MessageResponse> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(&client);
    project_dao.check_permission_by_uid(project_id, uid).await?;
    Ok(MessageResponse {
        message: "Check permission successfully".to_string(),
    })
}

pub async fn member(state: &AppState, project_id: &i32) -> AppResult<Vec<ProjectMember>> {
    let client = state.pool.get().await?;
    let project_dao = ProjectDao::new(&client);
    let members = project_dao.get_project_members(project_id).await?;
    Ok(members)
}
