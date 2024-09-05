use std::collections::HashMap;

use crate::errors::{AppError, AppResult, Resource, ResourceType};

use crate::dao::entity;
use crate::dao::entity::ElementInfo;
use db::queries::element::*;

#[derive(Debug)]
pub struct ElementDao<'a, T>
where
    T: db::GenericClient,
{
    pub executor: &'a T,
}

impl<'a, T> ElementDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        ElementDao { executor }
    }

    pub async fn create(&self, element: entity::Element) -> AppResult<i32> {
        let element_id = insert()
            .bind(
                self.executor,
                &element.name,
                &element.value,
                &element.element_type,
                &element.description,
                &element.created_by,
            )
            .one()
            .await?;
        Ok(element_id)
    }

    pub async fn get_element(&self, e_id: i32, option_id: i32) -> AppResult<ElementInfo> {
        let element = get_element()
            .bind(self.executor, &e_id, &option_id)
            .opt()
            .await?;
        match element {
            Some(e) => Ok(ElementInfo {
                name: e.name,
                element_type: e.element_type,
                action: e.action,
                selector: e.value,
            }),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }

    pub async fn count(
        &self,
        project_id: &i32,
        is_deleted: &bool,
    ) -> AppResult<HashMap<String, i64>> {
        let mut module_element_count: HashMap<String, i64> = HashMap::new();
        let _ = count()
            .bind(self.executor, is_deleted, project_id)
            .all()
            .await?
            .into_iter()
            .map(|item| module_element_count.insert(item.module_name.clone(), item.element_count))
            .collect::<Vec<_>>();
        Ok(module_element_count)
    }

    #[allow(dead_code)]
    pub async fn update(&self, element: entity::Element) -> AppResult<()> {
        update()
            .bind(
                self.executor,
                &element.name,
                &element.value,
                &element.element_type,
                &element.description,
                &element.updated_by,
                &element.id,
            )
            .await?;
        Ok(())
    }
}
