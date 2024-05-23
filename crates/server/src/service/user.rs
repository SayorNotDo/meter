/* 用户注册 */
pub async fn register(state: AppState, request: RegisterRequest) -> AppResult<Uuid> {
	info!("Register a new user request: {request:?}.");
	/* 验证注册用户的用户名与邮箱唯一性 */
	check_unique_username_or_email(&request.username, &request.email).await?;
	/* 创建用户 */
}

/* 用户是否已经登录 */
pub async fn is_login() {}

pub async fn check_unique_username_or_email(username: &str, email: &str) -> AppResult {

}


#[cfg(test)]
mod tests {
	use super::*;


	#[tokio::test]
	async fn test_check_unique_username_or_eamil() {
		let username = "unique_username";
		let email = "unique_email@test.com";

		result = check_unique_username_or_email(username, email).await;

		assert!(result);
	}
}