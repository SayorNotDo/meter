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

pub async fn check_user_permission(
    state: &AppState,
    uid: &Uuid,
    project_id: &i32,
    uri: &str,
    method: &str,
) -> AppResult<bool> {
    let client = state.pool.get().await?;
    let perm_dao = PermissionDao::new(&client);
    /* 獲取當前用戶的角色及權限*/
    let user_dao = UserDao::new(&client);
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
