use std::collections::HashMap;

use crate::{
    dao::entity::Script,
    dto::request::Issue,
    entity::{
        case::{CaseStatus, Field, FieldOption, FunctionalCase, Template, TemplateField},
        file::FileModule,
    },
    errors::{AppError, AppResult, Resource, ResourceType},
    utils,
};

use db::queries::{case::*, template::*};
use serde_json::from_value;
use tracing::info;
use uuid::Uuid;

use crate::dao::entity::{self, Step};

use super::entity::Machine;

#[derive(Debug)]
pub struct CaseDao<'a, T>
where
    T: db::GenericClient,
{
    executor: &'a T,
}

trait ToTemplate {
    fn to_template(&self) -> Template;
}

macro_rules! impl_to_template {
    ($($t:ty),*) => {
        $(
            impl ToTemplate for $t {
                fn to_template(&self) -> Template {
                    let updated_at = utils::time::to_utc_or_default(self.updated_at);
                    let created_at = utils::time::to_utc(self.created_at);
                    /* construct fields array */
                    let fields: Vec<TemplateField> = match from_value(self.fields.clone()) {
                        Ok(fields) => fields,
                        Err(_) => {
                            vec![]
                        }
                    };
                    Template {
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

    pub async fn get_template_project_id(&self, project_id: i32) -> AppResult<Template> {
        let ret = get_template_by_project_id()
            .bind(self.executor, &project_id)
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

    pub async fn get_templates(&self, _project_id: &i32) -> AppResult<Vec<Template>> {
        Ok(vec![])
    }

    pub async fn get_template_by_id(&self, template_id: i32) -> AppResult<Template> {
        let ret = get_template_by_id()
            .bind(self.executor, &template_id)
            .opt()
            .await?;

        match ret {
            Some(t) => Ok(t.to_template()),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![("file_type: template".into(), "not found".into())],
                resource_type: ResourceType::Template,
            })),
        }
    }

    pub async fn get_field_by_id(&self, field_id: i32) -> AppResult<Field> {
        let ret = get_field_by_id()
            .bind(self.executor, &field_id)
            .opt()
            .await?;
        match ret {
            Some(field) => {
                let options: Vec<FieldOption> =
                    from_value(field.options).unwrap_or_else(|_| vec![]);
                Ok(Field {
                    id: field.id,
                    name: field.name,
                    project_id: field.project_id,
                    field_type: field.field_type,
                    remark: field.remark,
                    internal: field.internal,
                    options,
                })
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::Field,
            })),
        }
    }

    pub async fn create_field(&self, field: Field, created_by: Uuid) -> AppResult<i32> {
        let id = create_field()
            .bind(
                self.executor,
                &field.name,
                &field.project_id,
                &field.field_type,
                &field.internal,
                &field.remark,
                &created_by,
            )
            .one()
            .await?;

        Ok(id)
    }

    pub async fn update_field(&self, field: Field, updated_by: Uuid) -> AppResult {
        update_field()
            .bind(
                self.executor,
                &field.name,
                &field.field_type,
                &field.remark,
                &updated_by,
                &field.id,
            )
            .await?;
        Ok(())
    }

    pub async fn soft_delete_field(&self, field_id: i32, deleted_by: Uuid) -> AppResult {
        soft_delete_field()
            .bind(self.executor, &deleted_by, &field_id)
            .await?;

        Ok(())
    }

    pub async fn get_fields(&self, project_id: i32) -> AppResult<Vec<Field>> {
        let fields = get_fields()
            .bind(self.executor, &project_id)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let options: Vec<FieldOption> = from_value(item.options).unwrap_or_else(|_| vec![]);
                Field {
                    id: item.id,
                    name: item.name.clone(),
                    project_id: item.project_id,
                    internal: item.internal,
                    remark: item.remark,
                    field_type: item.field_type,
                    options,
                }
            })
            .collect::<Vec<_>>();
        Ok(fields)
    }

    pub async fn insert_field_option(
        &self,
        field_id: i32,
        option: FieldOption,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let id = insert_field_option()
            .bind(
                self.executor,
                &field_id,
                &option.value,
                &option.position,
                &created_by,
            )
            .one()
            .await?;
        Ok(id)
    }

    pub async fn get_field_option_by_id(&self, option_id: i32) -> AppResult<FieldOption> {
        let field_option = get_field_option_by_id()
            .bind(self.executor, &option_id)
            .opt()
            .await?;
        match field_option {
            Some(o) => Ok(FieldOption {
                id: o.id,
                value: o.value,
                position: o.position,
            }),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }

    pub async fn get_options_by_field_id(&self, field_id: i32) -> AppResult<Vec<FieldOption>> {
        let options = get_options_by_field_id()
            .bind(self.executor, &field_id)
            .all()
            .await?
            .into_iter()
            .map(|item| FieldOption {
                id: item.id,
                value: item.value,
                position: item.position,
            })
            .collect::<Vec<_>>();

        Ok(options)
    }

    pub async fn update_field_option(
        &self,
        field_option: FieldOption,
        updated_by: Uuid,
    ) -> AppResult {
        update_field_option()
            .bind(
                self.executor,
                &field_option.value,
                &field_option.position,
                &updated_by,
                &field_option.id,
            )
            .await?;
        Ok(())
    }

    pub async fn soft_delete_field_option(&self, id: i32, deleted_by: Uuid) -> AppResult {
        soft_delete_field_option()
            .bind(self.executor, &deleted_by, &id)
            .await?;

        Ok(())
    }

    pub async fn soft_delete_field_option_by_field_id(
        &self,
        field_id: i32,
        deleted_by: Uuid,
    ) -> AppResult {
        soft_delete_field_option_by_field_id()
            .bind(self.executor, &deleted_by, &field_id)
            .await?;

        Ok(())
    }

    pub async fn get_functional_case_list(
        &self,
        module_ids: Vec<i32>,
        page_size: i64,
        offset: i64,
    ) -> AppResult<Vec<FunctionalCase>> {
        let case_list = get_functional_case_list()
            .bind(self.executor, &module_ids, &page_size, &offset)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = utils::time::to_utc(item.created_at);
                let updated_at = utils::time::to_utc_or_default(item.updated_at);
                let custom_fields: Vec<Field> = from_value(item.fields)?;
                let module: FileModule = from_value(item.module)?;
                Ok(FunctionalCase {
                    id: item.id,
                    name: item.name,
                    module,
                    template_id: item.template_id,
                    tags: item.tags,
                    status: CaseStatus::from_str(&item.status),
                    created_at,
                    created_by: item.created_by,
                    updated_at,
                    updated_by: item.updated_by,
                    custom_fields,
                    attach_info: None,
                })
            })
            .collect::<AppResult<Vec<_>>>()?;
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

    pub async fn get_functional_case_by_id(&self, case_id: &i32) -> AppResult<FunctionalCase> {
        let detail = get_functional_case_by_id()
            .bind(self.executor, case_id)
            .opt()
            .await?;
        match detail {
            Some(u) => {
                let created_at = utils::time::to_utc(u.created_at);
                let updated_at = utils::time::to_utc_or_default(u.updated_at);
                let module: FileModule = from_value(u.module)?;
                let case = FunctionalCase {
                    id: u.id,
                    name: u.name,
                    module,
                    template_id: u.template_id,
                    status: CaseStatus::from_str(&u.status),
                    tags: u.tags,
                    attach_info: u.attach_info,
                    created_at,
                    created_by: u.created_by,
                    updated_at,
                    updated_by: u.updated_by,
                    custom_fields: from_value::<Vec<Field>>(u.fields)?,
                };
                Ok(case)
            }
            None => Err(AppError::NotFoundError(Resource {
                resource_type: ResourceType::File,
                details: vec![],
            })),
        }
    }

    pub async fn insert_functional_case(
        &self,
        case: FunctionalCase,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let case_id = insert_functional_case()
            .bind(
                self.executor,
                &case.name,
                &case.module.id,
                &case.template_id,
                &case.tags,
                &created_by,
            )
            .one()
            .await?;

        Ok(case_id)
    }

    pub async fn insert_case_field_relation_with_text(
        &self,
        case_id: i32,
        field_id: i32,
        value: &str,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let id = insert_case_field_relation_with_text()
            .bind(self.executor, &case_id, &field_id, &value, &created_by)
            .one()
            .await?;
        Ok(id)
    }

    pub async fn insert_case_field_relation_with_option(
        &self,
        case_id: i32,
        field_id: i32,
        option_id: i32,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let id = insert_case_field_relation_with_option()
            .bind(self.executor, &case_id, &field_id, &option_id, &created_by)
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
