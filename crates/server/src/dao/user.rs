use std::vec;

use time::PrimitiveDateTime;
use chrono::{DateTime, Utc, NaiveDateTime};
use crypto_utils::sha::{Algorithm, CryptographicHash};
use serde::Serialize;
use tokio_postgres::error::DbError;
use uuid::Uuid;
use crate::utils;

use crate::dao::Entity;
use crate::errors::{AppError, AppResult, Resource, ResourceType};
use crate::utils::time::time_convert;

use super::base::BaseDao;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Entity for User {
    const RESOURCE: ResourceType = ResourceType::User;
}

impl User {
    pub fn new(username: &str, password: &str, email: Option<&str>, gen_uuid: bool) -> Self {
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

        let email = match email {
            None => "".to_string(),
            Some(email) => email.to_string()
        };

        // hash the password using SHA-512 algorithm and encode it into String.
        let hashed_password = hex::encode(CryptographicHash::hash(
            Algorithm::SHA512,
            password.as_bytes(),
        ));

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
            hashed_password,
            email,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug)]
pub struct UserDao {
    client: db::Client,
}

impl UserDao {
    pub fn new(client: db::Client) -> Self {
        UserDao { client }
    }

    pub async fn find_by_username(&self, username: &str) -> AppResult<User> {
        /* 通过用户名查询用户并返回 */
        let ret = db::queries::users::get_user_by_username()
            .bind(&self.client, &Some(username))
            .opt()
            .await?;
        match ret {
            Some(user) => {
                let u = User {
                    id: user.id,
                    username: user.username,
                    uuid: Uuid::nil(),
                    hashed_password: user.hashed_password.unwrap(),
                    email: user.email.unwrap(),
                    created_at: time_convert(user.created_at).unwrap(),
                    updated_at: time_convert(user.updated_at?).unwrap()
                };
                Ok(u)
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
        let user = db::queries::users::get_user_by_username()
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
        let user = db::queries::users::get_user_by_email()
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
        let _users = db::queries::users::get_users()
            .bind(&self.client)
            .all()
            .await
            .unwrap();

        Ok(vec![])
    }

    async fn insert(&self, object: &User) -> AppResult<i32> {
        let user_id = db::queries::users::insert_user()
            .bind(
                &self.client,
                &object.username,
                &object.hashed_password,
                &object.email,
                &object.uuid,
            )
            .one()
            .await?;
        Ok(user_id)
    }

    async fn get_by_id(&self, _id: i32) -> Result<User, DbError> {
        todo!()
    }

    async fn update(&self, _object: &User) -> Result<User, DbError> {
        todo!()
    }

    async fn delete_by_id(&self, _id: i32) -> Result<User, DbError> {
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
        let email: Option<&str> = Some("test_email@test.com");

        let username_expected = "medzik";

        let user = User::new(username, password, email, false);
        assert_eq!(user.username, username_expected)
    }

    #[test]
    fn test_password_hashed() {
        let username = "username";
        let password = "password";
        let email: Option<&str> = Some("test_email@test.com");

        let user = User::new(username, password, email, false);

        assert_ne!(user.hashed_password, password)
    }

    #[tokio::test]
    async fn test_insert() {
        let username = "test_username";
        let password = "test_password";
        let email: Option<&str> = Some("test_email@test.com");

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
        let email: Option<&str> = Some("test_email@test.com");

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
