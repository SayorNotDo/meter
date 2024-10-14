use crate::dao::entity::Permission;
use crate::errors::AppResult;

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

    pub async fn get_permission_by_role_id(&self, _role_id: i32) -> AppResult<Vec<Permission>> {
        Ok(vec![])
    }

    pub async fn get_permission_by_api(
        &self,
        _uri: &str,
        _method: &str,
    ) -> AppResult<Vec<Permission>> {
        Ok(vec![])
    }
}
