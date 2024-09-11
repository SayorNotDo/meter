use std::collections::HashMap;

use tracing::info;
use uuid::Uuid;

use crate::{
    dao::{entity::Plan, plan::PlanDao},
    dto::request::{CreatePlanRequest, PlanQueryParam},
    errors::AppResult,
    state::AppState,
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
    let is_deleted = if let Some(is_deleted) = param.is_deleted {
        is_deleted
    } else {
        false
    };
    Ok(HashMap::new())
}
