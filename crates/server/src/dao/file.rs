use crate::dao::entity;
use crate::dao::entity::FileModule;
use crate::errors::AppResult;
use db::queries::file::*;

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

impl_to_file_module!(GetFileModules);

pub struct FileDao<'a> {
    client: &'a db::Client,
}

impl<'a> FileDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        FileDao { client }
    }

    pub async fn get_file_modules(
        &self,
        project_id: &i32,
        module_type: &str,
    ) -> AppResult<Vec<FileModule>> {
        let file_modules = get_file_modules()
            .bind(self.client, project_id, &module_type)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_file_module())
            .collect::<Vec<_>>();
        Ok(file_modules)
    }
}
