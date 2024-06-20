use std::collections::HashMap;

use crate::{
    dao::entity::FieldOption,
    errors::{AppError, AppResult, Resource, ResourceType},
    utils,
};
use db::queries::{
    case::{count, get_case_list},
    template::*,
};
use serde_json::from_value;
use tracing::info;

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
                    let updated_at = utils::time::to_utc_or_default(self.updated_at);
                    let created_at = utils::time::to_utc(self.created_at);
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
                        created_at,
                        updated_at,
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
                    default_value: None,
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
        module_id: &Vec<i32>,
        page_size: &i64,
        offset: &i64,
    ) -> AppResult<Vec<CaseInfo>> {
        let case_list = get_case_list()
            .bind(self.client, project_id, module_id, page_size, offset)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = utils::time::to_utc(item.created_at);
                let updated_at = utils::time::to_utc_or_default(item.updated_at);
                let custom_fields: Vec<entity::CustomField> =
                    match from_value(item.custom_fields.clone()) {
                        Ok(fields) => fields,
                        Err(_) => {
                            vec![]
                        }
                    };
                CaseInfo {
                    id: item.id,
                    name: item.name,
                    module_id: item.module_id,
                    template_id: item.template_id,
                    tags: item.tags,
                    status: item.status,
                    created_at,
                    created_by: item.created_by,
                    updated_at,
                    updated_by: item.updated_by,
                    custom_fields,
                }
            })
            .collect::<Vec<_>>();
        Ok(case_list)
    }

    pub async fn count(
        &self,
        project_id: &i32,
        is_deleted: &bool,
    ) -> AppResult<HashMap<String, i64>> {
        let mut module_case_count: HashMap<String, i64> = HashMap::new();
        let _ = count()
            .bind(self.client, project_id, is_deleted)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                info!("{item:?}");
                module_case_count.insert(item.module_name.clone(), item.case_count)
            })
            .collect::<Vec<_>>();
        Ok(module_case_count)
    }
}
