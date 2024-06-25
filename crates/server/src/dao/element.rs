use crate::errors::AppResult;

use super::entity;

#[derive(Debug)]
pub struct ElementDao<'a> {
    pub client: &'a db::Client,
}

impl<'a> ElementDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        ElementDao { client }
    }

    pub async fn create(&self, _element: entity::Element) -> AppResult<i32> {
        Ok(0)
    }
}
