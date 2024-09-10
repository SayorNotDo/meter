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
    _state: &AppState,
    _project_id: &i32,
    _param: &PlanQueryParam,
) -> AppResult<HashMap<String, i64>> {
    Ok(HashMap::new())
}
