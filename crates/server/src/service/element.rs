use std::collections::HashMap;

use chrono::Utc;
use tracing::info;
use uuid::Uuid;

use crate::{
    dao::{element::ElementDao, entity::Element},
    dto::{
        request::{CreateElementRequest, ElementQueryParam},
        response::ElementResponse,
    },
    errors::AppResult,
    state::AppState,
};

pub async fn create(state: &AppState, uid: Uuid, request: CreateElementRequest) -> AppResult {
    let client = state.pool.get().await?;
    // let transaction = client.transaction().await?;
    let element_dao = ElementDao::new(&client);
    let element = Element::new(
        &request.name,
        &request.value,
        &request.element_type,
        request.description.as_deref(),
        uid,
    );
    let _element_id = element_dao.create(element).await?;
    Ok(())
}

/* Element exec main logic */
#[allow(dead_code)]
pub async fn exec(state: &AppState, script_id: i32) -> AppResult {
    info!("execute script: {script_id:?}");
    let mut client = state.pool.get().await?;
    let _element_dao = ElementDao::new(&mut client);
    /* get script by id and scan for environment requirement */
    // let script = element_dao.get_script_by_id();
    /* traverse steps field for executing */
    Ok(())
}

pub async fn list(_state: &AppState, _project_id: i32) -> AppResult<ElementResponse> {
    Ok(ElementResponse {
        id: 0,
        name: "".into(),
        description: Option::None,
        element_type: "TEXT".into(),
        value: "".into(),
        created_at: Utc::now(),
        created_by: "".into(),
        updated_at: Option::None,
        updated_by: Option::None,
    })
}

pub async fn count(
    state: &AppState,
    project_id: &i32,
    param: &ElementQueryParam,
) -> AppResult<HashMap<String, i64>> {
    info!("service layer for element count with project_id: {project_id:?} & params: {param:?}");
    let client = state.pool.get().await?;
    let element_dao = ElementDao::new(&client);
    let is_deleted = if let Some(is_deleted) = param.is_deleted {
        is_deleted
    } else {
        false
    };
    let hmap = element_dao.count(project_id, &is_deleted).await?;
    Ok(hmap)
}
