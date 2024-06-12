use axum::{Extension, Json};
use axum::extract::Query;

use crate::dto::request::CaseTreeQueryParam;
use crate::dto::response::DirectoryResponse;
use crate::errors::AppResult;
use crate::service::user::info;
use crate::state::AppState;
use tracing::info;
use crate::service::case;

#[utoipa::path(
    get,
    path = "/case/module/tree",
    params(CaseTreeQueryParam),
    responses(
        (status = 200, description = "Get case tree", body = [()]),
        (status = 401, description = "Unauthorized user", body = [AppResponseError]),
        (status = 404, description = "case tree not found", body = [AppResponseError]),
        (status = 500, description = "Internal server error", body = [AppResponseError]),
    ),
    security(("jwt" = []))
)]
pub async fn tree(
    Extension(state): Extension<AppState>,
    Query(param): Query<CaseTreeQueryParam>,
) -> AppResult<Json<DirectoryResponse>> {
    info!("case module tree query param: {param:?}");
    match case::tree(&state, &param.project_id).await {
        Ok(resp) => {
            Ok(Json(resp))
        }
        Err(e) => {
            info!("Failed to get case module tree");
            Err(e)
        }
    }
}