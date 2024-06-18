use crate::{
    dao::entity::FieldOption,
    errors::{AppError, AppResult, Resource, ResourceType},
};
use chrono::DateTime;
use db::queries::{
    case::{count, get_case_list},
    template::*,
};
use serde_json::from_value;

use super::entity::{self, CaseInfo};

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
                    let custom_fields: Vec<entity::CustomField> = match from_value(self.custom_fields.clone()) {
                        Ok(fields) => fields,
                        Err(_) => {
                            vec![]
                        }
                    };
                    entity::Template {
                        id: self.id,
                        name: self.name.clone(),
                        internal: self.internal,
                        description: self.description.clone(),
                        created_by: self.created_by.clone(),
                        created_at: DateTime::from_timestamp_nanos(timestamp_updated_at as i64),
                        updated_at: Option::from(DateTime::from_timestamp_nanos(timestamp_created_at as i64)),
                        custom_fields,
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
                let template = t.to_template();
                Ok(template)
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }

    pub async fn get_fields(
        &self,
        project_id: &i32,
        internal: bool,
    ) -> AppResult<Vec<entity::CustomField>> {
        let fields = get_fields()
            .bind(self.client, project_id, &internal)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let options: Vec<FieldOption> = match from_value(item.options) {
                    Ok(s) => s,
                    Err(_) => vec![],
                };
                entity::CustomField {
                    id: item.id,
                    name: item.name.clone(),
                    internal: item.internal,
                    field_type: item.field_type,
                    required: false,
                    options,
                }
            })
            .collect::<Vec<_>>();
        Ok(fields)
    }

    pub async fn get_case_list(
        &self,
        project_id: &i32,
        page_size: &i64,
        offset: &i64,
    ) -> AppResult<Vec<CaseInfo>> {
        let case_list = get_case_list()
            .bind(self.client, project_id, page_size, offset)
            .all()
            .await?
            .into_iter()
            .map(|item| CaseInfo {})
            .collect::<Vec<_>>();
        Ok(case_list)
    }

    pub async fn count(&self, project_id: &i32) -> AppResult<i64> {
        let count = count().bind(self.client, project_id).one().await?;
        Ok(count)
    }
}
