use std::collections::HashMap;

use crate::utils::time;
use db::queries::plan::*;
use tracing::info;

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

    pub async fn count_by_module_id(&self, module_id: &i32, is_deleted: bool) -> AppResult<i32> {
        let count = count_by_module_id()
            .bind(self.executor, module_id, &is_deleted)
            .opt()
            .await?;
        match count {
            Some(c) => Ok(c as i32),
            None => Ok(0),
        }
    }

    #[allow(dead_code)]
    pub async fn count(
        &self,
        project_id: &i32,
        is_deleted: bool,
    ) -> AppResult<HashMap<String, i64>> {
        let mut plan_module_count: HashMap<String, i64> = HashMap::new();
        let _ = count()
            .bind(self.executor, &is_deleted, project_id)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                info!("{item:?}");
                plan_module_count.insert(item.module_name.clone(), item.plan_count)
            })
            .collect::<Vec<_>>();
        Ok(plan_module_count)
    }
}
