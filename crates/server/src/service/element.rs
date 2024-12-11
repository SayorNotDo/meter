use std::collections::HashMap;

use tracing::info;
use uuid::Uuid;

use crate::{
    constant::PAGE_DECODE_KEY,
    dao::{element::ElementDao, entity::Element, file::FileDao},
    dto::{
        request::{CreateElementRequest, ElementQueryParam, ListQueryParam},
        response::ListElementResponse,
    },
    errors::AppResult,
    service::token::generate_page_token,
    state::AppState,
    utils::claim::PageClaims,
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

pub async fn list(
    state: &AppState,
    project_id: &i32,
    param: &ListQueryParam,
) -> AppResult<ListElementResponse> {
    info!("service layer for list with project_id: {project_id:?}, param: {param:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let (page_size, page_num, last_item_id) = match &param.page_token {
        Some(token) => {
            let page_claims = PageClaims::decode(token.as_str(), &PAGE_DECODE_KEY)?.claims;
            (
                page_claims.page_size,
                page_claims.page_num,
                page_claims.last_item_id,
            )
        }
        None => {
            let page_size = param.page_size.unwrap_or(10);
            (page_size, 1, 0)
        }
    };
    let module_id = if let Some(id) = param.module_id {
        vec![id]
    } else {
        let file_dao = FileDao::new(&transaction);
        file_dao
            .get_root_module_id(project_id, "ELEMENT".into())
            .await?
    };
    let element_dao = ElementDao::new(&transaction);
    let list = element_dao
        .get_element_list(&module_id, &page_size, &last_item_id)
        .await?;
    let next_cursor = match list.last() {
        Some(l) => l.id,
        None => 0,
    };
    let next_page_token = generate_page_token(page_size, page_num + 1, next_cursor)?;
    transaction.commit().await?;
    Ok(ListElementResponse {
        next_page_token,
        list,
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
