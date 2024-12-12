use std::collections::HashMap;

use tracing::info;
use uuid::Uuid;

use crate::{
    dao::{file::FileDao, plan::PlanDao},
    dto::{
        request::{CreatePlanRequest, ListQueryParam, PlanQueryParam},
        response::ListPlanResponse,
    },
    entity::project::Plan,
    errors::AppResult,
    service::token::{generate_page_token, parse_page_token},
    state::AppState,
    utils::{claim::PageClaims, parse_ids},
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

pub async fn get_plan_list(
    state: &AppState,
    project_id: &i32,
    param: ListQueryParam,
) -> AppResult<ListPlanResponse> {
    info!("service layer for list with project_id: {project_id:?}, param: {param:?}");
    let mut client = state.pool.get().await?;
    let transaction = client.transaction().await?;
    let plan_dao = PlanDao::new(&transaction);
    let page_claims = match param.page_token {
        Some(page_token) => parse_page_token(page_token)?,
        None => {
            let page_size = param.page_size.unwrap_or(10).clamp(1, 100);
            let page_num = param.page_num.unwrap_or(1).max(1);
            let offset = (page_num - 1) * page_size;
            let last_item_id = if offset > 0 {
                plan_dao.get_query_cursor(offset).await?
            } else {
                0
            };
            let module_ids = match param.module_ids {
                Some(ids) => parse_ids(&ids)?,
                None => {
                    let file_dao = FileDao::new(&transaction);
                    file_dao
                        .get_all_module_id(project_id, "PLAN".into())
                        .await?
                }
            };
            PageClaims::new(page_size, page_num, last_item_id, module_ids)
        }
    };
    let list = plan_dao
        .get_plan_list(
            &page_claims.module_ids,
            &page_claims.page_size,
            &page_claims.last_item_id,
        )
        .await?;
    let next_cursor = match list.last() {
        Some(l) => l.id,
        None => 0,
    };
    let next_page_token = generate_page_token(
        page_claims.page_size,
        page_claims.page_num + 1,
        next_cursor,
        page_claims.module_ids,
    )?;
    transaction.commit().await?;
    Ok(ListPlanResponse {
        next_page_token,
        list,
    })
}
