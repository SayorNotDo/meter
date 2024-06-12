use axum::Json;
use crate::errors::AppResult;

#[utoipa::path(
    get,
    path = "/case/tree",
    responses(
        (status = 200, description = "Get case tree", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case tree not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn tree() -> AppResult<Json<()>> {
    Ok(Json(()))
}