use std::vec;

use crate::dao::entity;
use crate::errors::{AppError, AppResult, Resource, ResourceType};
use crate::utils::time;
use chrono::DateTime;
use db::queries::user::*;
use tracing::log::info;
use uuid::Uuid;

trait ToUser {
    fn to_user(&self) -> entity::User;
}

macro_rules! impl_to_user {
    ($include_hashed_password:expr, $($t:ty),*) => {
        $(
        impl ToUser for $t {
            fn to_user(&self) -> entity::User {
                let timestamp_updated_at = match self.updated_at {
                    Some(t) => t.assume_utc().unix_timestamp_nanos(),
                    None => 0
                };
                let timestamp_created_at = self.created_at.assume_utc().unix_timestamp_nanos();
                let hashed_password = if $include_hashed_password {
                    self.hashed_password.clone()
                } else { "".into() };
                entity::User {
                    id: self.id,
                    username: self.username.clone(),
                    uuid: self.uuid,
                    hashed_password,
                    email: self.email.clone(),
                    enable: self.enable,
                    created_at: DateTime::from_timestamp_nanos(timestamp_created_at as i64),
                    updated_at: Option::from(DateTime::from_timestamp_nanos(timestamp_updated_at as i64)),
                    last_project_id: self.last_project_id
                }
            }
        }
        )*
    };
}

// 使用宏为查询的结构体实现ToUser trait
impl_to_user!(true, GetUserByUsername, GetUserByUuid, GetUserById);
impl_to_user!(
    false,
    GetUsersByRoleAndProjectId,
    GetUsers,
    GetIdleUsersByProjectId
);

#[derive(Debug)]
pub struct UserDao<'a, T>
where
    T: db::GenericClient,
{
    executor: &'a T,
}

impl<'a, T> UserDao<'a, T>
where
    T: db::GenericClient,
{
    pub fn new(executor: &'a T) -> Self {
        UserDao { executor }
    }
    pub async fn find_by_uid(&self, uid: &Uuid) -> AppResult<entity::User> {
        /* 通过uid查询用户 */
        let ret = get_user_by_uuid().bind(self.executor, uid).opt().await?;
        match ret {
            Some(user) => {
                let user = user.to_user();
                info!("Successfully find by uid: {user:?}.");
                Ok(user)
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::User,
            })),
        }
    }

    pub async fn find_by_id(&self, id: &i32) -> AppResult<entity::User> {
        /* 通过主键查询用户 */
        let ret = get_user_by_id().bind(self.executor, id).opt().await?;
        match ret {
            Some(user) => {
                let user = user.to_user();
                info!("Successfully find user id: {user:?}");
                Ok(user)
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::User,
            })),
        }
    }

    pub async fn update_user(&self, username: &str, email: &str, uid: i32) -> AppResult {
        update_user()
            .bind(self.executor, &username, &email, &uid)
            .await?;

        Ok(())
    }

    pub async fn soft_deleted_user(&self, operator: Uuid, uid: &i32) -> AppResult {
        soft_delete_user()
            .bind(self.executor, &operator, uid)
            .await?;
        Ok(())
    }

    pub async fn batch_update_user_status(&self, enable: bool, uid_list: Vec<i32>) -> AppResult {
        update_status()
            .bind(self.executor, &enable, &uid_list)
            .await?;
        Ok(())
    }

    pub async fn find_by_role_and_project_id(
        &self,
        role: &str,
        project_id: i32,
    ) -> AppResult<Vec<entity::User>> {
        /*  通过项目id和角色id查询用户 */
        let users = get_users_by_role_and_project_id()
            .bind(self.executor, &project_id, &role)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_user())
            .collect::<Vec<_>>();
        Ok(users)
    }

    pub async fn find_by_username(&self, username: String) -> AppResult<entity::User> {
        /* 通过用户名查询用户并返回 */
        let ret = get_user_by_username()
            .bind(self.executor, &username)
            .opt()
            .await?;
        match ret {
            Some(user) => {
                let user = user.to_user();
                info!("Successfully find by name: {user:?}.");
                Ok(user)
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::User,
            })),
        }
    }

    pub async fn check_unique_by_username(&self, username: &str) -> AppResult {
        let user = get_user_by_username()
            .bind(self.executor, &username)
            .opt()
            .await?;
        match user {
            None => Ok(()),
            Some(_) => Err(AppError::ResourceExistsError(Resource {
                details: vec![],
                resource_type: ResourceType::User,
            })),
        }
    }

    pub async fn check_unique_by_email(&self, email: &str) -> AppResult {
        let user = get_user_by_email()
            .bind(self.executor, &Some(email))
            .opt()
            .await?;
        match user {
            None => Ok(()),
            Some(_) => Err(AppError::ResourceExistsError(Resource {
                details: vec![],
                resource_type: ResourceType::User,
            })),
        }
    }

    pub async fn check_role_unique_by_name(&self, role_name: &str) -> AppResult {
        let role = get_user_role_by_name()
            .bind(self.executor, &role_name)
            .opt()
            .await?;

        match role {
            None => Ok(()),
            Some(_) => Err(AppError::ResourceExistsError(Resource {
                details: vec![],
                resource_type: ResourceType::Role,
            })),
        }
    }

    pub async fn get_user_roles_by_uuid(&self, uuid: &Uuid) -> AppResult<Vec<entity::UserRole>> {
        let mut ret = vec![];
        let user_roles = get_user_roles_by_uuid()
            .bind(self.executor, uuid)
            .all()
            .await?;
        for item in user_roles {
            let timestamp_updated_at = match item.updated_at {
                Some(t) => t.assume_utc().unix_timestamp_nanos(),
                None => 0,
            };
            let timestamp_created_at = item.created_at.assume_utc().unix_timestamp_nanos();
            let user_role = entity::UserRole {
                id: item.id,
                name: item.name,
                role_type: item.role_type,
                internal: item.internal,
                created_at: DateTime::from_timestamp_nanos(timestamp_created_at as i64),
                created_by: item.created_by,
                updated_at: Option::from(DateTime::from_timestamp_nanos(
                    timestamp_updated_at as i64,
                )),
                description: item.description,
            };
            ret.push(user_role);
        }
        Ok(ret)
    }

    pub async fn get_role_by_uuid_and_project_id(
        &self,
        uuid: &Uuid,
        project_id: &i32,
    ) -> AppResult<entity::UserRole> {
        let ret = get_user_role_by_uuid_and_project_id()
            .bind(self.executor, uuid, project_id)
            .opt()
            .await?;
        match ret {
            Some(role) => {
                let created_at = time::to_utc(role.created_at);
                let updated_at = time::to_utc_or_default(role.updated_at);
                Ok(entity::UserRole {
                    id: role.id,
                    name: role.name,
                    role_type: role.role_type,
                    internal: role.internal,
                    created_at,
                    created_by: role.created_by,
                    updated_at,
                    description: role.description,
                })
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::Role,
            })),
        }
    }

    pub async fn get_user_role_list_by_project_id(
        &self,
        project_id: &i32,
    ) -> AppResult<Vec<entity::UserRoleOption>> {
        let mut ret = vec![];
        let user_roles = get_user_role_list_by_project_id()
            .bind(self.executor, project_id)
            .all()
            .await?;
        for item in user_roles {
            let user_role = entity::UserRoleOption {
                id: item.id,
                name: item.name,
            };
            ret.push(user_role);
        }
        Ok(ret)
    }

    #[allow(dead_code)]
    pub async fn get_idle_users_by_project_id(
        &self,
        project_id: &i32,
    ) -> AppResult<Vec<entity::User>> {
        let users = get_idle_users_by_project_id()
            .bind(self.executor, project_id)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_user())
            .collect::<Vec<_>>();
        Ok(users)
    }

    pub async fn get_role_by_id(&self, role_id: i32) -> AppResult<entity::UserRole> {
        let ret = get_role_by_id().bind(self.executor, &role_id).opt().await?;
        match ret {
            Some(role) => {
                let created_at = time::to_utc(role.created_at);
                let updated_at = time::to_utc_or_default(role.updated_at);
                Ok(entity::UserRole {
                    id: role.id,
                    name: role.name,
                    role_type: role.role_type,
                    internal: role.internal,
                    created_at,
                    created_by: role.created_by,
                    updated_at,
                    description: None,
                })
            }
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::Role,
            })),
        }
    }

    pub async fn get_user_role_relations_by_uuid(
        &self,
        uuid: &Uuid,
    ) -> AppResult<Vec<entity::UserRoleRelation>> {
        let mut ret = vec![];
        let user_role_relations = get_user_role_relations_by_uuid()
            .bind(self.executor, uuid)
            .all()
            .await?;
        for item in user_role_relations {
            let timestamp_created_at = item.created_at.assume_utc().unix_timestamp_nanos();
            let user_role = entity::UserRoleRelation {
                id: item.id,
                user_id: item.user_id,
                role_id: item.role_id,
                project_id: item.project_id,
                created_by: item.created_by,
                created_at: DateTime::from_timestamp_nanos(timestamp_created_at as i64),
            };
            ret.push(user_role);
        }
        Ok(ret)
    }

    pub async fn get_user_role_permissions_by_role_id(
        &self,
        _role_id: &i32,
    ) -> AppResult<Vec<entity::Permission>> {
        let ret = vec![];
        // let user_role_permissions = get_user_role_permissions_by_role_id()
        //     .bind(self.executor, role_id)
        //     .all()
        //     .await?;
        // for item in user_role_permissions {
        //     let permission = entity::Permission {
        //         id: item.id,
        //         module: "".to_string(),
        //         scope: item.permission,
        //     };
        //     ret.push(permission);
        // }
        Ok(ret)
    }

    pub async fn insert(&self, object: &entity::User) -> AppResult<i32> {
        let user_id = insert_user()
            .bind(
                self.executor,
                &object.username,
                &object.hashed_password,
                &object.email,
                &object.uuid,
            )
            .one()
            .await?;
        Ok(user_id)
    }

    pub async fn insert_role(
        &self,
        name: String,
        role_type: String,
        description: Option<String>,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let role_id = insert_user_role()
            .bind(self.executor, &name, &role_type, &description, &created_by)
            .one()
            .await?;
        Ok(role_id)
    }

    pub async fn insert_user_role_relation(
        &self,
        uid: Uuid,
        role_id: i32,
        project_id: i32,
        created_by: Uuid,
    ) -> AppResult<i32> {
        let relation_id = insert_user_role_relation()
            .bind(self.executor, &uid, &role_id, &project_id, &created_by)
            .one()
            .await?;
        Ok(relation_id)
    }

    pub async fn all(&self) -> AppResult<Vec<entity::User>> {
        let users = get_users()
            .bind(self.executor)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_user())
            .collect::<Vec<_>>();

        Ok(users)
    }
}
