use crate::dao::entity::UserRolePermission;
use crate::{
    dao::{entity::Permission, permission::PermissionDao, user::UserDao},
    errors::AppResult,
    state::AppState,
};
use uuid::Uuid;

#[allow(dead_code)]
pub async fn get_role_permission(state: &AppState, role_id: i32) -> AppResult<Vec<Permission>> {
    let client = state.pool.get().await?;
    let perm_dao = PermissionDao::new(&client);

    let permission_list = perm_dao.get_permission_by_role_id(role_id).await?;

    Ok(permission_list)
}

pub async fn get_role_permission_list(state: &AppState) -> AppResult<Vec<UserRolePermission>> {
    let client = state.pool.get().await?;

    Ok(vec![])
}

pub async fn check_user_permission(
    state: &AppState,
    uid: &Uuid,
    project_id: &i32,
    uri: &str,
    method: &str,
) -> AppResult<bool> {
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    /* 判断当前用户是否处于可用状态 */
    let user = user_dao.find_by_uid(uid).await?;
    if !user.enable {
        return Ok(false);
    }
    let perm_dao = PermissionDao::new(&client);
    /* 获取当前用戶的角色及对应的权限 */
    let role = user_dao
        .get_role_by_uuid_and_project_id(uid, project_id)
        .await?;
    let role_permission_list = perm_dao.get_permission_by_role_id(role.id).await?;
    /* 查询请求API所需的权限列表 */
    let api_permission_list = perm_dao.get_permission_by_api(uri, method).await?;
    let enable = api_permission_list
        .iter()
        .all(|item| role_permission_list.contains(item));

    Ok(enable)
}
