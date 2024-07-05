use std::collections::HashMap;

use crate::{
    dao::entity::{CustomField, FieldOption, Script},
    errors::{AppError, AppResult, Resource, ResourceType},
    utils,
};
use db::queries::{case::*, template::*};
use serde_json::from_value;
use tracing::info;

use crate::dao::entity::{self, CaseDetail, FunctionalCase, Step};

#[derive(Debug)]
pub struct CaseDao<'a, T>
where
    T: db::GenericClient,
{
    executor: &'a T,
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

impl<'a, T> CaseDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        CaseDao { executor }
    }

    pub async fn get_template(
        &self,
        project_id: &i32,
        internal: bool,
    ) -> AppResult<entity::Template> {
        let ret = get_template_by_project_id()
            .bind(self.executor, project_id, &internal)
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
            .bind(self.executor, project_id, &internal)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let options: Vec<FieldOption> = from_value(item.options).unwrap_or_else(|_| vec![]);
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
    ) -> AppResult<Vec<CaseDetail>> {
        let case_list = get_case_list()
            .bind(self.executor, project_id, module_id, page_size, offset)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = utils::time::to_utc(item.created_at);
                let updated_at = utils::time::to_utc_or_default(item.updated_at);
                let custom_fields: Vec<entity::CustomField> =
                    from_value(item.custom_fields.clone()).unwrap_or_else(|_| vec![]);
                CaseDetail {
                    id: item.id,
                    name: item.name,
                    module_name: item.module_name,
                    template_id: item.template_id,
                    tags: item.tags,
                    status: item.status,
                    created_at,
                    created_by: item.created_by,
                    updated_at,
                    updated_by: item.updated_by,
                    custom_fields,
                    attach_info: None,
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
            .bind(self.executor, project_id, is_deleted)
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

    pub async fn count_by_module_id(
        &self,
        project_id: &i32,
        module_id: &i32,
        is_deleted: &bool,
    ) -> AppResult<i64> {
        let count = count_by_module_id()
            .bind(self.executor, project_id, module_id, is_deleted)
            .opt()
            .await?;
        match count {
            Some(c) => Ok(c),
            None => Ok(0),
        }
    }

    pub async fn detail(&self, case_id: &i32) -> AppResult<entity::CaseDetail> {
        let detail = detail().bind(self.executor, case_id).opt().await?;
        match detail {
            Some(u) => {
                let created_at = utils::time::to_utc(u.created_at);
                let updated_at = utils::time::to_utc_or_default(u.updated_at);
                let case = CaseDetail {
                    id: u.id,
                    name: u.name,
                    module_name: u.module_name,
                    template_id: u.template_id,
                    status: u.status,
                    tags: u.tags,
                    attach_info: u.attach_info,
                    created_at,
                    created_by: u.created_by,
                    updated_at,
                    updated_by: u.updated_by,
                    custom_fields: from_value::<Vec<entity::CustomField>>(u.custom_fields)?,
                };
                Ok(case)
            }
            None => Err(AppError::NotFoundError(Resource {
                resource_type: ResourceType::File,
                details: vec![],
            })),
        }
    }

    pub async fn insert_functional_case(&self, case: FunctionalCase) -> AppResult<i32> {
        let case_id = insert_functional_case()
            .bind(
                self.executor,
                &case.name,
                &case.module_id,
                &case.template_id,
                &case.tags,
                &case.created_by,
            )
            .one()
            .await?;

        Ok(case_id)
    }

    pub async fn insert_case_field_relation(
        &self,
        case_id: i32,
        fields: Vec<CustomField>,
    ) -> AppResult<()> {
        for item in fields.iter() {
            insert_case_field_relation()
                .bind(self.executor, &case_id, &item.id, &item.default_value)
                .await?;
        }
        Ok(())
    }

    pub async fn insert_script(&self, script: Script) -> AppResult<i32> {
        let ret = insert_script()
            .bind(
                self.executor,
                &script.case_id,
                &script.environment,
                &script.path,
                &script.created_by,
            )
            .one()
            .await?;
        Ok(ret)
    }

    pub async fn insert_script_element_relation(
        &self,
        script_id: &i32,
        field_type: String,
        steps: &Vec<Step>,
    ) -> AppResult<()> {
        for item in steps.iter() {
            let serialized = serde_json::to_string(&item.attach_info)?;
            let _ = insert_script_element_relation()
                .bind(
                    self.executor,
                    script_id,
                    &field_type,
                    &item.option_id,
                    &item.element_id,
                    &item.position,
                    &serialized,
                )
                .one()
                .await?;
        }
        Ok(())
    }
}
