use crate::utils::time;
use db::queries::plan::insert;

use crate::{dao::entity::Plan, errors::AppResult};

pub struct PlanDao<'a, T>
where
    T: db::GenericClient,
{
    pub executor: &'a T,
}

impl<'a, T> PlanDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        PlanDao { executor }
    }

    pub async fn create(&self, plan: Plan) -> AppResult<i32> {
        let plan_id = insert()
            .bind(
                self.executor,
                &plan.name,
                &plan.project_id,
                &plan.description,
                &plan.module_id,
                &plan.created_by,
                &time::to_date_or_default(plan.start_date),
                &time::to_date_or_default(plan.end_date),
            )
            .one()
            .await?;
        Ok(plan_id)
    }
}
