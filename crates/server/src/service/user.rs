use crate::dao;
use crate::dto::request::*;
use crate::errors::AppResult;
use crate::state::AppState;

use tracing::info;
// use uuid::Uuid;

/* 用户注册 */
pub async fn register(state: AppState, request: RegisterRequest) -> AppResult<i32> {
    info!("Register a new user request: {request:?}.");
    /* 验证注册用户的用户名与邮箱唯一性 */
    check_unique_username_or_email(&state.pool, &request.username, &request.email).await?;
    /* 创建用户 */
    Ok(0)
}

/* 用户是否已经登录 */
pub async fn is_login() {}

pub async fn check_unique_username_or_email(
    pool: &db::Pool,
    username: &str,
    email: &str,
) -> AppResult {
    let client = pool.get().await.unwrap();
    let user_dao = dao::user::UserDao::new(client);
    user_dao.check_unique_by_username(username).await?;
    user_dao.check_unique_by_email(email).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_unique_username_or_eamil() {
        let username = "unique_username";
        let email = "unique_email@test.com";

        let pool = db::create_pool(
            "postgresql://postgres:testpassword@192.168.50.234:5432/postgres?sslmode=disable",
        );

        let result = check_unique_username_or_email(&pool, username, email).await;

        assert!(result.is_ok());
    }
}
