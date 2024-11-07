use crate::{dao::entity::Permission, errors::AppResult};
use db::queries::permission::*;

#[derive(Debug)]
pub struct PermissionDao<'a, T>
where
    T: db::GenericClient,
{
    pub executor: &'a T,
}

impl<'a, T> PermissionDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        PermissionDao { executor }
    }

    pub async fn get_permission_by_role_id(&self, role_id: i32) -> AppResult<Vec<Permission>> {
        let permission_list = get_permission_by_role_id()
            .bind(self.executor, &role_id)
            .all()
            .await?
            .into_iter()
            .map(|item| Permission {
                id: item.id,
                module: item.module,
                scope: item.scope,
            })
            .collect::<Vec<_>>();
        Ok(permission_list)
    }

    pub async fn get_permission_by_api(
        &self,
        uri: &str,
        method: &str,
    ) -> AppResult<Vec<Permission>> {
        let permission_list = get_permission_by_api()
            .bind(self.executor, &uri, &method)
            .all()
            .await?
            .into_iter()
            .map(|item| Permission {
                id: item.id,
                module: item.module,
                scope: item.scope,
            })
            .collect::<Vec<_>>();
        Ok(permission_list)
    }
}
