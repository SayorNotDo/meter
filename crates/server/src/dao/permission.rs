use crate::{
    dao::entity::{Permission, UserRole, UserRolePermission},
    errors::AppResult,
    utils::time,
};
use tracing::error;

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

    pub async fn get_permission_group_by_role(&self) -> AppResult<Vec<UserRolePermission>> {
        let role_permission_list = get_permission_group_by_role()
            .bind(self.executor)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = time::to_utc(item.created_at);
                let updated_at = time::to_utc_or_default(item.updated_at);
                let permission_list: Vec<Permission> =
                    serde_json::from_str(&item.permission_list.to_string()).unwrap_or_else(|e| {
                        error!("Get empty permission list with exception: {e}");
                        vec![]
                    });
                /* TODO: error while permission_list is empty cause not allowed. */
                UserRolePermission {
                    user_role: UserRole {
                        id: item.id,
                        name: item.role_name,
                        role_type: item.role_type,
                        internal: item.internal,
                        created_at,
                        created_by: item.created_by,
                        updated_at,
                        description: item.description,
                    },
                    permission_list,
                }
            })
            .collect::<Vec<_>>();
        Ok(role_permission_list)
    }
}
