/// 用户注册
#[utoipa::path(
	post,
	request_body = RegisterRequest,
	path = "/auth/register",
	response(
		(status = 200, description = "Success register user", body = [RegisterResponse]),
		(status = 400, description = "Invalid data input", body = [CustomError]),
		(status = 500, description = "Internal server error", body = [CustomError])
	)
)]
pub async fn register() -> 

/* 用户是否已经登录 */
pub async fn is_login() {}
