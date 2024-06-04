use std::vec;

use chrono::{Utc, DateTime};
use serde::Serialize;
use tokio_postgres::error::DbError;
use tracing::log::info;
use uuid::Uuid;
use db::queries::user::*;
use crate::errors::{AppError, AppResult, Resource, ResourceType};

use super::base::BaseDao;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_organization_id: Option<i32>,
    pub last_project_id: Option<i32>,
}

impl User {
    pub fn new(username: &str, password: &str, email: &str, gen_uuid: bool) -> Self {
        let username = username.to_lowercase();

        // generate UUID
        let uuid = if gen_uuid {
            Uuid::new_v4()
        } else {
            Uuid::nil()
        };

        Self {
            id: 0,
            uuid,
            username,
            hashed_password: password.to_string(),
            email: email.into(),
            created_at: Utc::now(),
            updated_at: None,
            last_project_id: None,
            last_organization_id: None,
        }
    }
}

trait ToUser {
    fn to_user(&self) -> User;
}

macro_rules! impl_to_user {
    ($($t:ty),*) => {
        $(
        impl ToUser for $t {
            fn to_user(&self) -> User {
                let timestamp_updated_at = match self.updated_at {
                    Some(t) => t.assume_utc().unix_timestamp_nanos(),
                    None => 0
                };
                let timestamp_created_at = self.created_at.assume_utc().unix_timestamp_nanos();
                User {
                    id: self.id,
                    username: self.username.clone(),
                    uuid: self.uuid,
                    hashed_password: self.hashed_password.clone(),
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
impl_to_user!(GetUserByUsername, GetUserByUuid);

#[derive(Debug)]
pub struct UserDao {
    client: db::Client,
}

impl UserDao {
    pub fn new(client: db::Client) -> Self {
        UserDao { client }
    }
    pub async fn find_by_uid(&self, uid: Uuid) -> AppResult<User> {
        /* 通过uid查询用户 */
        let ret = get_user_by_uuid()
            .bind(&self.client, &Some(uid))
            .opt()
            .await?;
        match ret {
            Some(user) => {
                let user = user.to_user();
                info!("Successfully find by uid: {user:?}.");
                Ok(user)
            }
            None => {
                Err(AppError::NotFoundError(Resource {
                    details: vec![],
                    resource_type: ResourceType::User,
                }))
            }
        }
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<User> {
        /* 通过用户名查询用户并返回 */
        let ret = get_user_by_username()
            .bind(&self.client, &Some(username))
            .opt()
            .await?;
        match ret {
            Some(user) => {
                let user = user.to_user();
                info!("Successfully find by name: {user:?}.");
                Ok(user)
            }
            None => {
                Err(AppError::NotFoundError(Resource {
                    details: vec![],
                    resource_type: ResourceType::User,
                }))
            }
        }
    }

    pub async fn check_unique_by_username(&self, username: &str) -> AppResult {
        let user = get_user_by_username()
            .bind(&self.client, &Some(username))
            .opt()
            .await?;
        match user {
            None => Ok(()),
            Some(_) => {
                Err(AppError::ResourceExistsError(Resource {
                    details: vec![],
                    resource_type: ResourceType::User,
                }))
            }
        }
    }

    pub async fn check_unique_by_email(&self, email: &str) -> AppResult {
        let user = get_user_by_email()
            .bind(&self.client, &Some(email))
            .opt()
            .await?;
        match user {
            None => Ok(()),
            Some(_) => {
                Err(AppError::ResourceExistsError(Resource {
                    details: vec![],
                    resource_type: ResourceType::User,
                }))
            }
        }
    }
}

impl BaseDao<User> for UserDao {
    async fn all(&self) -> AppResult<Vec<User>> {
        let _users = get_users()
            .bind(&self.client)
            .all()
            .await
            .unwrap();

        Ok(vec![])
    }

    async fn insert(&self, object: User) -> AppResult<i32> {
        let user_id = insert_user()
            .bind(
                &self.client,
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
    async fn find_by_id(&self, _id: i32) -> AppResult<User> {
        todo!()
    }

    #[allow(dead_code)]
    async fn update(&self, _object: &User) -> Result<User, DbError> {
        todo!()
    }

    #[allow(dead_code)]
    async fn delete_by_id(&self, _id: i32) -> AppResult<()> {
        todo!()
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

        let user = User::new(username, password, email, false);
        assert_eq!(user.username, username_expected)
    }

    #[test]
    fn test_password_hashed() {
        let username = "username";
        let password = "password";
        let email = "test_email@test.com";


        let user = User::new(username, password, email, false);

        assert_ne!(user.hashed_password, password)
    }

    #[tokio::test]
    async fn test_insert() {
        let username = "test_username";
        let password = "test_password";
        let email = "test_email@test.com";

        let user = User::new(username, password, email, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();

        let user_dao = UserDao::new(client);
        let result = user_dao.insert(&user).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_complicated_username() {
        let username = "test_username";
        let password = "test_password";
        let email = "test_email@test.com";

        let user = User::new(username, password, email, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();
        let user_dao = UserDao::new(client);
        let result = user_dao.insert(&user).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_get_users() {
        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let client = pool.get().await.unwrap();
        let user_dao = UserDao::new(client);

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
        let user_dao = UserDao::new(client);

        let result = user_dao.check_unique_by_username(&username).await;

        // assert_eq!(result, true)
        dbg!(())
    }
}
