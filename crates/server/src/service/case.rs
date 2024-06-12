use crate::dto::response::DirectoryResponse;
use crate::state::AppState;

pub async fn tree(state: &AppState, project_id: &i32) -> DirectoryResponse{
    let client = state.pool.get().await?;

}