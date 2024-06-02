use crate::dto::response::ProjectInfoResponse;
use crate::errors::AppResult;


/* 获取项目信息 */
pub async fn info() -> AppResult<ProjectInfoResponse> {
    Ok(ProjectInfoResponse {})
}