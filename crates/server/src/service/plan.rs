use std::collections::HashMap;

use tracing::info;
use uuid::Uuid;

use crate::{
    constant::PAGE_DECODE_KEY,
    dao::{file::FileDao, plan::PlanDao},
    dto::{
        request::{CreatePlanRequest, ListQueryParam, PlanQueryParam},
        response::ListPlanResponse,
    },
    entity::project::Plan,
    errors::AppResult,
    service::token::generate_page_token,
    state::AppState,
    utils::claim::PageClaims,
};

pub async fn create(state: &AppState, uid: Uuid, request: CreatePlanRequest) -> AppResult {
    info!("service layer create plan with request_body: {request:?} created_by: {uid}");
    let mut client = state.pool.get().await?;
    let plan_dao = PlanDao::new(&mut client);
    let plan = Plan::new(
        &request.name,
        request.project_id,
        request.module_id,
        uid,
        request.description,
        request.start_date,
        request.end_date,
    );
    let _plan_id = plan_dao.create(plan).await?;
    Ok(())
}

pub async fn count(
    state: &AppState,
    project_id: &i32,
    param: &PlanQueryParam,
) -> AppResult<HashMap<String, i64>> {
    info!("service layer for plan count with project_id: {project_id}, param: {param:?}");
    let mut client = state.pool.get().await?;
    let plan_dao = PlanDao::new(&mut client);
    let is_deleted = match param.is_deleted {
        Some(deleted) => deleted,
        None => false,
    };
    let hmap = plan_dao.count(project_id, is_deleted).await?;
    Ok(hmap)
}

pub async fn list(
    state: &AppState,
    project_id: &i32,
    param: &ListQueryParam,
) -> AppResult<ListPlanResponse> {
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
            .get_root_module_id(project_id, "PLAN".into())
            .await?
    };
    info!("===>> module_id: {module_id:?}");
    let plan_dao = PlanDao::new(&transaction);
    let list = plan_dao
        .get_plan_list(&module_id, &page_size, &last_item_id)
        .await?;
    let next_cursor = match list.last() {
        Some(l) => l.id,
        None => 0,
    };
    let next_page_token = generate_page_token(page_size, page_num + 1, next_cursor)?;
    transaction.commit().await?;
    Ok(ListPlanResponse {
        next_page_token,
        list,
    })
}
