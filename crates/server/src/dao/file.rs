use crate::{
    dao::entity::{self, FileModule},
    errors::{AppError, AppResult, Resource, ResourceType},
};

use db::queries::file::*;
use uuid::Uuid;

trait ToFileModule {
    fn to_file_module(&self) -> entity::FileModule;
}

macro_rules! impl_to_file_module {
    ($($t:ty), *) => {
        $(
        impl ToFileModule for $t {
            fn to_file_module(&self) -> FileModule {
                    FileModule {
                        id: self.id,
                        name: self.name.clone(),
                        position: self.position,
                        module_type: self.module_type.clone(),
                        parent_id: self.parent_id,
                    }
                }
            }
        )*
    };
}

impl_to_file_module!(GetFileModules, GetFileModuleById);

pub struct FileDao<'a, T>
where
    T: db::GenericClient,
{
    executor: &'a T,
}

impl<'a, T> FileDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        FileDao { executor }
    }

    pub async fn get_file_modules(
        &self,
        project_id: &i32,
        module_type: &str,
    ) -> AppResult<Vec<FileModule>> {
        let file_modules = get_file_modules()
            .bind(self.executor, project_id, &module_type)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_file_module())
            .collect::<Vec<_>>();
        Ok(file_modules)
    }

    pub async fn get_root_module_id(
        &self,
        project_id: &i32,
        module_type: &str,
    ) -> AppResult<Vec<i32>> {
        let module_id_list = get_root_module()
            .bind(self.executor, project_id, &module_type)
            .all()
            .await?;
        Ok(module_id_list)
    }

    pub async fn insert_file_module(
        &self,
        uid: &Uuid,
        project_id: i32,
        file_module: &FileModule,
    ) -> AppResult<i32> {
        let module_id = insert_file_module()
            .bind(
                self.executor,
                &project_id,
                &file_module.name,
                &file_module.position,
                &file_module.module_type,
                &file_module.parent_id,
                uid,
            )
            .one()
            .await?;
        Ok(module_id)
    }

    pub async fn get_module_by_id(&self, module_id: i32) -> AppResult<FileModule> {
        let ret = get_file_module_by_id()
            .bind(self.executor, &module_id)
            .opt()
            .await?;
        match ret {
            Some(module) => Ok(module.to_file_module()),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::File,
            })),
        }
    }
}
