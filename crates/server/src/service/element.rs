use tracing::info;
use uuid::Uuid;

use crate::{
    dao::{element::ElementDao, entity::Element},
    dto::{request::CreateElementRequest, response::ElementResponse},
    errors::AppResult,
    state::AppState,
};

pub async fn create(
    state: &AppState,
    uid: Uuid,
    request: CreateElementRequest,
) -> AppResult<ElementResponse> {
    let client = state.pool.get().await?;
    let element_dao = ElementDao::new(&client);
    let element = Element::new(
        &request.name,
        &request.value,
        &request.element_type,
        request.description.as_deref(),
        uid,
    );
    let _element_id = element_dao.create(element).await?;
    Ok(ElementResponse {})
}

/* Element exec main logic */
#[allow(dead_code)]
pub async fn exec(state: &AppState, script_id: i32) -> AppResult {
    info!("execute script: {script_id:?}");
    let client = state.pool.get().await?;
    let _element_dao = ElementDao::new(&client);
    /* get script by id and scan for environment requirement */
    // let script = element_dao.get_script_by_id();
    /* traverse steps field for executing */
    Ok(())
}
