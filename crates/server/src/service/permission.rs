use crate::{
    dao::{entity::Permission, permission::PermissionDao},
    errors::AppResult,
    state::AppState,
};

#[allow(dead_code)]
pub async fn get_role_permission(state: &AppState, role_id: i32) -> AppResult<Vec<Permission>> {
    let client = state.pool.get().await?;
    let perm_dao = PermissionDao::new(&client);

    let permisson_list = perm_dao.get_permission_by_role_id(role_id).await?;

    Ok(permisson_list)
}

#[allow(dead_code)]
pub async fn check_user_permission() -> AppResult {
    Ok(())
}
