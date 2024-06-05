use tracing::info;
use uuid::Uuid;

use crate::dao;
use crate::dao::base::BaseDao;
use crate::dao::user::UserDao;
use crate::dto::request::*;
use crate::dto::response::{LoginResponse, UserInfoResponse};
use crate::dto::response::MessageResponse;
use crate::errors::AppResult;
use crate::service::redis::SessionKey;
use crate::service::session;
use crate::service::token;
use crate::state::AppState;
use crate::utils;

/* 用户注册 */
pub async fn register(state: &AppState, request: RegisterRequest) -> AppResult<i32> {
    info!("Register a new user request: {request:?}.");
    /* 验证注册用户的用户名与邮箱唯一性 */
    check_unique_username_or_email(&state.pool, &request.username, &request.email).await?;
    /* 创建用户 */
    let hashed_password = utils::password::hash(request.password).await?;
    let new_user = dao::user::User::new(
        &request.username,
        &hashed_password,
        &request.email,
        true,
    );
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(client);
    let user_id = user_dao.insert(new_user).await?;
    Ok(user_id)
}

/* 用户登录 */
pub async fn login(state: &AppState, request: LoginRequest) -> AppResult<LoginResponse> {
    info!("User login request: {request:?}.");
    let client = state.pool.get().await?;
    let user_dao = UserDao::new(client);
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
        message: ("Successfully logout".to_string()),
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
    let user_dao = UserDao::new(client);
    /* 查询用户相关的信息，组装响应返回 */
    let user = user_dao.find_by_uid(uid).await?;
    Ok(UserInfoResponse {
        username: user.username,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
        last_organization_id: user.last_organization_id,
        last_project_id: user.last_project_id,
        user_roles: Vec::new(),
        user_role_permissions: Vec::new(),
        user_role_relations: Vec::new(),
    })
}

pub async fn check_unique_username_or_email(
    pool: &db::Pool,
    username: &str,
    email: &str,
) -> AppResult {
    let client = pool.get().await?;
    let user_dao = dao::user::UserDao::new(client);
    user_dao.check_unique_by_username(username).await?;
    user_dao.check_unique_by_email(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_unique_username_or_email() {
        let username = "unique_username";
        let email = "unique_email@test.com";

        let pool = db::create_pool(
            "postgresql://postgres:testpassword@192.168.50.234:5432/postgres?sslmode=disable",
        );

        let result = check_unique_username_or_email(&pool, username, email).await;

        assert!(result.is_ok());
    }
}
