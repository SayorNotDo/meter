use crate::errors::{AppResponseError, AppResult};

use crate::dao::entity;
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

    pub async fn get_element(&self, e_id: i32, option_id: i32) -> AppResult<()> {
        let element = get_element()
            .bind(self.client, &e_id, &option_id)
            .one()
            .await?;
        Ok(())
    }

    pub async fn insert_elements(&mut self) -> AppResult<()> {
        Ok(())
    }
}
