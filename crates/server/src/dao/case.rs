use crate::errors::{AppError, AppResult, Resource, ResourceType};
use chrono::DateTime;
use tracing::info;
use db::queries::template::*;

use super::entity;

#[derive(Debug)]
pub struct CaseDao<'a> {
    client: &'a db::Client,
}

trait ToTemplate {
    fn to_template(&self) -> entity::Template;
}

macro_rules! impl_to_template {
    ($($t:ty),*) => {
        $(
            impl ToTemplate for $t {
                fn to_template(&self) -> entity::Template {
                    let timestamp_updated_at = match self.updated_at {
                        Some(t) => t.assume_utc().unix_timestamp_nanos(),
                        None => 0
                    };
                    let timestamp_created_at = self.created_at.assume_utc().unix_timestamp_nanos();
                    /* construct customs fields array */
                    entity::Template {
                        id: self.id,
                        name: self.name.clone(),
                        internal: self.internal,
                        description: self.description.clone(),
                        created_by: self.created_by.clone(),
                        created_at: DateTime::from_timestamp_nanos(timestamp_updated_at as i64),
                        updated_at: Option::from(DateTime::from_timestamp_nanos(timestamp_created_at as i64)),
                        custom_fields: Vec::new()
                    }
                }
            }
        )*
    };
}

impl_to_template!(GetTemplateByProjectId);

impl<'a> CaseDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        CaseDao { client }
    }

    pub async fn get_template(
        &self,
        project_id: &i32,
        internal: bool,
    ) -> AppResult<entity::Template> {
        let ret = get_template_by_project_id()
            .bind(self.client, project_id, &internal)
            .opt()
            .await?;
        match ret {
            Some(t) => {
                info!("template: {t:?}");
                let template = t.to_template();
                Ok(template)
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }

    // pub async fn get_custom_field(
    //     &self,
    //     template_id: &i32
    // ) -> AppResult<entity::CustomField> {
    //     let ret = get_template_custom_field()
    //         .bind(self.client, template_id)
    //         .opt()
    //         .await?;
    // }
}
