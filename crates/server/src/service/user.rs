use tracing::info;
use uuid::Uuid;

use crate::{
    dao::{
        self,
        entity::{User, UserRoleOption, UserRolePermission},
        user::UserDao,
    },
    dto::{
        request::*,
        response::{LoginResponse, MessageResponse, UserInfoResponse},
    },
    errors::AppResult,
    service::{redis::SessionKey, session, token},
    state::AppState,
    utils,
};

/* 用户注册 */
pub async fn batch_register(state: &AppState, request: RegisterRequest) -> AppResult<()> {
    info!("Register a new user request: {request:?}.");
    /* TODO: 新增逻辑批量创建用户 */
    for item in request.user_info_list {
        register(state, item.username, item.email).await?;
    }
    Ok(())
}

/* 单个用户注册 */
pub async fn register(state: &AppState, username: String, email: String) -> AppResult<()> {
    info!("Register new user with username: {username}, email: {email}");
    check_unique_username_or_email(state, &username, &email).await?;
    /* 生成随机密码 */
    let password = utils::password::generate().await?;
    let hashed_password = utils::password::hash(password.clone()).await?;
    let new_user = dao::entity::User::new(&username, &hashed_password, &email, true);
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    user_dao.insert(new_user).await?;
    /* 增加邮件发送逻辑 */
    utils::smtp::registered_inform(&username, &password)?;
    Ok(())
}

pub async fn update_status(state: &AppState, request: UserStatusRequest) -> AppResult {
    info!("service layer update user status by field `enable`");
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    user_dao
        .batch_update_user_status(request.enable, request.select_ids)
        .await?;
    Ok(())
}

/* 用户登录 */
pub async fn login(state: &AppState, request: LoginRequest) -> AppResult<LoginResponse> {
    info!("User login request: {request:?}.");
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    let user = user_dao.find_by_username(&request.username).await?;
    /* 校验用户密码 */
    utils::password::verify(request.password.clone(), user.hashed_password.clone()).await?;
    /* 生成token */
    let session_id = session::set(&state.redis, user.uuid).await?;
    let resp = token::generate_tokens(user.uuid, session_id)?;
    Ok(LoginResponse::Token(resp))
}

/* 用户登出 */
pub async fn logout(state: &AppState, uid: Uuid) -> AppResult<MessageResponse> {
    info!("User logout");
    let key = SessionKey { uuid: uid };
    crate::service::redis::del(&state.redis, &key).await?;
    Ok(MessageResponse {
        message: "Successfully logout".to_string(),
    })
}

/* 用户是否已经登录 */
pub async fn is_login(state: &AppState, uid: Uuid) -> AppResult<LoginResponse> {
    info!("Check whether user is login");
    let key = SessionKey { uuid: uid };
    crate::service::redis::get(&state.redis, &key).await?;
    let session_id = session::set(&state.redis, uid).await?;
    let resp = token::generate_tokens(uid, session_id)?;
    Ok(LoginResponse::Token(resp))
}

pub async fn info(state: &AppState, uid: Uuid) -> AppResult<UserInfoResponse> {
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    /* 查询用户相关的信息，组装响应返回 */
    let user = user_dao.find_by_uid(&uid).await?;
    /* 获取用户角色信息 */
    let user_roles = user_dao.get_user_roles_by_uuid(&uid).await?;
    /* 获取用户角色关系信息 */
    let user_role_relations = user_dao.get_user_role_relations_by_uuid(&uid).await?;
    /* 获取用户角色对应的权限 */
    let mut permissions_list = vec![];
    for item in user_roles.iter() {
        let permissions = user_dao
            .get_user_role_permissions_by_role_id(&item.id)
            .await?;
        let user_role_permissions = UserRolePermission {
            user_role: item.clone(),
            user_role_permissions: permissions,
        };
        permissions_list.push(user_role_permissions);
    }
    Ok(UserInfoResponse {
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_project_id: user.last_project_id,
        user_roles,
        user_role_permissions: permissions_list,
        user_role_relations,
    })
}

pub async fn list(state: &AppState, _uid: Uuid) -> AppResult<Vec<User>> {
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    let users = user_dao.all().await?;
    Ok(users)
}

pub async fn role_list(state: &AppState, project_id: i32) -> AppResult<Vec<UserRoleOption>> {
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);

    let role_list = user_dao
        .get_user_role_list_by_project_id(&project_id)
        .await?;
    Ok(role_list)
}

pub async fn check_unique_username_or_email(
    state: &AppState,
    username: &str,
    email: &str,
) -> AppResult {
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(&client);
    user_dao.check_unique_by_username(username).await?;
    user_dao.check_unique_by_email(email).await
}

#[cfg(test)]
mod tests {}
