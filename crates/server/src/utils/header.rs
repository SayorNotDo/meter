use crate::{
    constant::PROJECT_ID,
    errors::{AppError, AppResult},
};

use axum::http::HeaderMap;

pub fn validate_project_id(headers: &HeaderMap, project_id: i32) -> AppResult {
    let id = extract_project_id(headers)?;

    if id != project_id {
        return Err(AppError::ForbiddenError("Access denied".to_string()));
    }
    Ok(())
}

pub fn extract_project_id(headers: &HeaderMap) -> AppResult<i32> {
    let header_value = headers
        .get(PROJECT_ID)
        .ok_or_else(|| AppError::ForbiddenError("Missing ProjectId header".to_string()))?;

    let id_str = header_value
        .to_str()
        .map_err(|_| AppError::ForbiddenError("Invalid ProjectId header format".to_string()))?;

    let id = id_str.parse::<i32>().map_err(|_| {
        AppError::ForbiddenError("ProjectId header is not valid integer".to_string())
    })?;

    Ok(id)
}
