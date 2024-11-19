use std::collections::HashMap;

use crate::{
    dao::entity::{FieldOption, Script},
    dto::request::{case::SelectedField, Issue},
    errors::{AppError, AppResult, Resource, ResourceType},
    utils,
};
use db::queries::{case::*, template::*};
use serde_json::from_value;
use tracing::info;
use uuid::Uuid;

use crate::dao::entity::{self, CaseDetail, FunctionalCase, Step};

use super::entity::Machine;

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
                    /* construct fields array */
                    let fields: Vec<entity::TemplateField> = match from_value(self.fields.clone()) {
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
                        fields,
                    }
                }
            }
        )*
    };
}

impl_to_template!(GetTemplateByProjectId, GetTemplateById);

impl<'a, T> CaseDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        CaseDao { executor }
    }

    pub async fn delete_by_module_id(&self, deleted_by: &Uuid, module_id: &i32) -> AppResult {
        let _ = delete_by_module_id()
            .bind(self.executor, deleted_by, module_id)
            .await?;
        Ok(())
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

    pub async fn get_template_by_id(&self, template_id: i32) -> AppResult<entity::Template> {
        let ret = get_template_by_id()
            .bind(self.executor, &template_id)
            .opt()
            .await?;

        match ret {
            Some(t) => Ok(t.to_template()),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![("file_type: template".into(), "not found".into())],
                resource_type: ResourceType::File,
            })),
        }
    }

    pub async fn get_field_by_id(&self, field_id: i32) -> AppResult<entity::Field> {
        let ret = get_field_by_id()
            .bind(self.executor, &field_id)
            .opt()
            .await?;
        match ret {
            Some(field) => {
                let options: Vec<FieldOption> =
                    from_value(field.options).unwrap_or_else(|_| vec![]);
                Ok(entity::Field {
                    id: field.id,
                    name: field.name,
                    field_type: field.field_type,
                    internal: field.internal,
                    options,
                })
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
    ) -> AppResult<Vec<entity::Field>> {
        let fields = get_fields()
            .bind(self.executor, project_id, &internal)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let options: Vec<FieldOption> = from_value(item.options).unwrap_or_else(|_| vec![]);
                entity::Field {
                    id: item.id,
                    name: item.name.clone(),
                    internal: item.internal,
                    field_type: item.field_type,
                    options,
                }
            })
            .collect::<Vec<_>>();
        Ok(fields)
    }

    pub async fn get_case_list(
        &self,
        module_id: &Vec<i32>,
        page_size: &i64,
        offset: &i64,
    ) -> AppResult<Vec<CaseDetail>> {
        let case_list = get_case_list()
            .bind(self.executor, module_id, page_size, offset)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = utils::time::to_utc(item.created_at);
                let updated_at = utils::time::to_utc_or_default(item.updated_at);
                let custom_fields: Vec<entity::Field> =
                    from_value(item.fields.clone()).unwrap_or_else(|_| vec![]);
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

    pub async fn count(&self, project_id: &i32) -> AppResult<HashMap<String, i64>> {
        let mut module_case_count: HashMap<String, i64> = HashMap::new();
        let _ = count()
            .bind(self.executor, project_id)
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

    pub async fn count_deleted_case(&self, project_id: &i32) -> AppResult<HashMap<String, i64>> {
        let mut module_case_count: HashMap<String, i64> = HashMap::new();
        let _ = count_deleted_case()
            .bind(self.executor, project_id)
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

    pub async fn count_by_module_id(&self, module_id: &i32) -> AppResult<i32> {
        let count = count_by_module_id()
            .bind(self.executor, module_id)
            .opt()
            .await?;
        match count {
            Some(c) => Ok(c as i32),
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
                    custom_fields: from_value::<Vec<entity::Field>>(u.fields)?,
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
        field: SelectedField,
    ) -> AppResult<i32> {
        let id = insert_case_field_relation()
            .bind(
                self.executor,
                &case_id,
                &field.field_id,
                &field.value,
                &field.option_id,
            )
            .one()
            .await?;

        Ok(id)
    }

    pub async fn insert_case_issue_relation(
        &self,
        case_id: &i32,
        issue: &Issue,
        created_by: &Uuid,
    ) -> AppResult {
        let _ = insert_case_issue_relation()
            .bind(
                self.executor,
                case_id,
                &issue.issue_id,
                &issue.source,
                &issue.uri,
                created_by,
            )
            .await?;
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
    ) -> AppResult {
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

    pub async fn get_machine(&self, machine_id: &i32) -> AppResult<entity::Machine> {
        match get_machine().bind(self.executor, machine_id).opt().await? {
            Some(m) => Ok(Machine {
                addr: m.addr,
                authentication: m.authentication,
                user: "".into(),
                password: "".into(),
            }),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }
}
