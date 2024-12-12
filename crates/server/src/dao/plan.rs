use std::collections::HashMap;

use crate::{entity::project::Plan, errors::AppResult, utils};
use db::queries::plan::*;
use tracing::info;

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
                &utils::time::to_date_or_default(plan.start_date),
                &utils::time::to_date_or_default(plan.end_date),
            )
            .one()
            .await?;
        Ok(plan_id)
    }

    pub async fn get_query_cursor(&self, offset: i64) -> AppResult<i32> {
        let id = get_query_cursor()
            .bind(self.executor, &offset)
            .one()
            .await?;
        Ok(id)
    }

    pub async fn get_plan_list(
        &self,
        module_id: &Vec<i32>,
        page_size: &i64,
        last_item_id: &i32,
    ) -> AppResult<Vec<Plan>> {
        let plan_list = get_plan_list()
            .bind(self.executor, module_id, last_item_id, page_size)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = utils::time::to_utc(item.created_at);
                let updated_at = utils::time::to_utc_or_default(item.updated_at);
                Plan {
                    id: item.id,
                    name: item.name,
                    status: item.status,
                    description: item.description,
                    module_id: item.module_id,
                    project_id: item.project_id,
                    created_at,
                    created_by: item.created_by,
                    updated_at,
                    updated_by: item.updated_by,
                    start_date: utils::time::date_to_utc(item.start_date),
                    end_date: utils::time::date_to_utc(item.end_date),
                }
            })
            .collect::<Vec<_>>();

        Ok(plan_list)
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
