use std::borrow::BorrowMut;

use crate::errors::{AppResponseError, AppResult};

use super::entity::{self, StepInfo};

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

    pub async fn get_elements(&self, ids: Vec<i32>) -> AppResult<()> {
        Ok(())
    }

    pub async fn insert_elements(&mut self) -> AppResult<()> {
        let transaction = self.client.borrow_mut().transaction().await?;
        Ok(())
    }
}
