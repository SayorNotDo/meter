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

    pub async fn create(&self, _element: entity::Element) -> AppResult<i32> {
        let id = insert().
            bind(self.executor, )
        Ok(0)
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

    #[allow(dead_code)]
    pub async fn insert_elements(&self) -> AppResult<()> {
        Ok(())
    }
}
