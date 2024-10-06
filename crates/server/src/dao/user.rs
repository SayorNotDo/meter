use std::vec;

use crate::dao::entity;
use crate::errors::{AppError, AppResult, Resource, ResourceType};
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
                    created_at: DateTime::from_timestamp_nanos(timestamp_created_at as i64),
                    updated_at: Option::from(DateTime::from_timestamp_nanos(timestamp_updated_at as i64)),
                    last_project_id: self.last_project_id,
                    last_organization_id: self.last_organization_id
                }
            }
        }
        )*
    };
}

// 使用宏为查询的结构体实现ToUser trait
impl_to_user!(true, GetUserByUsername, GetUserByUuid);
impl_to_user!(false, GetUsersByRoleAndProjectId, GetUsers);

#[derive(Debug)]
pub struct UserDao<'a> {
    client: &'a db::Client,
}

impl<'a> UserDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        UserDao { client }
    }
    pub async fn find_by_uid(&self, uid: &Uuid) -> AppResult<entity::User> {
        /* 通过uid查询用户 */
        let ret = get_user_by_uuid().bind(self.client, uid).opt().await?;
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

    pub async fn find_by_role_and_project_id(
        &self,
        role: &str,
        project_id: i32,
    ) -> AppResult<Vec<entity::User>> {
        /*  通过项目id和角色id查询用户 */
        let users = get_users_by_role_and_project_id()
            .bind(self.client, &project_id, &role)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_user())
            .collect::<Vec<_>>();
        Ok(users)
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<entity::User> {
        /* 通过用户名查询用户并返回 */
        let ret = get_user_by_username()
            .bind(self.client, &Some(username))
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
            .bind(self.client, &Some(username))
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
            .bind(self.client, &Some(email))
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

    pub async fn get_user_roles_by_uuid(&self, uuid: &Uuid) -> AppResult<Vec<entity::UserRole>> {
        let mut ret = vec![];
        let user_roles = get_user_roles_by_uuid()
            .bind(self.client, uuid)
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

    pub async fn get_user_role_list_by_project_id(
        &self,
        project_id: &i32,
    ) -> AppResult<Vec<entity::UserRoleOption>> {
        let mut ret = vec![];
        let user_roles = get_user_role_list_by_project_id()
            .bind(self.client, project_id)
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

    pub async fn get_user_role_relations_by_uuid(
        &self,
        uuid: &Uuid,
    ) -> AppResult<Vec<entity::UserRoleRelation>> {
        let mut ret = vec![];
        let user_role_relations = get_user_role_relations_by_uuid()
            .bind(self.client, uuid)
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
        role_id: &i32,
    ) -> AppResult<Vec<entity::Permission>> {
        let mut ret = vec![];
        let user_role_permissions = get_user_role_permissions_by_role_id()
            .bind(self.client, role_id)
            .all()
            .await?;
        for item in user_role_permissions {
            let permission = entity::Permission {
                id: item.id,
                role_id: item.role_id,
                permission: item.permission,
            };
            ret.push(permission);
        }
        Ok(ret)
    }

    pub async fn insert(&self, object: entity::User) -> AppResult<i32> {
        let user_id = insert_user()
            .bind(
                self.client,
                &object.username.as_str(),
                &object.hashed_password.as_str(),
                &object.email.as_str(),
                &object.uuid,
            )
            .one()
            .await?;
        Ok(user_id)
    }

    #[allow(dead_code)]
    pub async fn all(&self) -> AppResult<Vec<entity::User>> {
        let users = get_users()
            .bind(self.client)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_user())
            .collect::<Vec<_>>();

        Ok(users)
    }
}

#[cfg(test)]
mod tests {

    // use crate::service::user::check_unique_username_or_email;

    use super::*;

    #[test]
    fn test_username_is_lowercase() {
        let username = "MeDZik";
        let password = "password";
        let email = "test_email@test.com";

        let username_expected = "medzik";

        let user = entity::User::new(username, password, email, false);
        assert_eq!(user.username, username_expected)
    }

    #[test]
    fn test_password_hashed() {
        let username = "username";
        let password = "password";
        let email = "test_email@test.com";

        let user = entity::User::new(username, password, email, false);

        assert_ne!(user.hashed_password, password)
    }

    #[tokio::test]
    async fn test_insert() {
        let username = "test_username";
        let password = "test_password";
        let email = "test_email@test.com";

        let user = entity::User::new(username, password, email, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();

        let user_dao = UserDao::new(&client);
        let result = user_dao.insert(user).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_complicated_username() {
        let username = "test_username";
        let password = "test_password";
        let email = "test_email@test.com";

        let user = entity::User::new(username, password, email, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();
        let user_dao = UserDao::new(&client);
        let result = user_dao.insert(user).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_get_users() {
        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();
        let user_dao = UserDao::new(&client);

        let _result = user_dao.all().await;
        // assert!(result.ok())
        dbg!(())
    }

    #[tokio::test]
    async fn test_check_unique_by_username() {
        let username = "test_unique_username";

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();
        let user_dao = UserDao::new(&client);

        let _result = user_dao.check_unique_by_username(&username).await;

        // assert_eq!(result, true)
        dbg!(())
    }
}
