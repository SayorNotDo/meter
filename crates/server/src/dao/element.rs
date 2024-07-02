use crate::errors::{AppError, AppResult, Resource, ResourceType};

use crate::dao::entity;
use crate::dao::entity::ElementInfo;
use db::queries::element::*;

#[derive(Debug)]
pub struct ElementDao<'a> {
    pub client: &'a mut db::Client,
}

impl<'a> ElementDao<'a> {
    pub fn new(client: &'a mut db::Client) -> Self {
        ElementDao { client }
    }

    pub async fn create(&self, _element: entity::Element) -> AppResult<i32> {
        Ok(0)
    }

    pub async fn get_element(&self, e_id: i32, option_id: i32) -> AppResult<ElementInfo> {
        let element = get_element()
            .bind(self.client, &e_id, &option_id)
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
    pub async fn insert_elements(&mut self) -> AppResult<()> {
        Ok(())
    }
}
