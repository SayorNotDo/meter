use crate::errors::CustomError;
use axum::Extension;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio_postgres::error::DbError;

use crypto_utils::sha::{Algorithm, CryptographicHash};
use uuid::Uuid;

use super::base::BaseDao;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct User {
    pub id: i32,
    pub uuid: Uuid,
    pub username: String,
    pub hashed_password: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: &str, password: &str, gen_uuid: bool) -> Self {
        let username = username.to_lowercase();

        // salting the password
        let password = format!("{username}${password}");

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
            email: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub struct UserDao {
    pool: db::Pool,
}

impl UserDao {
    pub fn new(Extension(pool): Extension<db::Pool>) -> Self {
        UserDao { pool }
    }

    async fn check_unique_by_username(&self, username: &str) -> bool {
        let client = self.pool.get().await.unwrap();

        let user = db::queries::users::get_user_by_username()
            .bind(&client, &username)
            .all()
            .await
            .unwrap();

        user.is_empty()
    }

    async fn check_unique_by_email(_email: &str) -> bool {
        false
    }
}

impl BaseDao<User> for UserDao {
    // async fn all(&self) -> Result<Json<Vec<User>>, CustomError> {
    //     let client = self.pool.get().await.unwrap();
    //     let users = db::queries::users::get_users()
    //         .bind(&client)
    //         .all()
    //         .await
    //         .unwrap();

    //     Ok(Json(users))
    // }

    async fn get_by_id(&self, _id: i32) -> Result<User, DbError> {
        todo!()
    }

    async fn insert(&self, object: &User) -> Result<(), CustomError> {
        let client = self.pool.get().await.unwrap();
        let ret = db::queries::users::insert_user()
            .bind(
                &client,
                &object.username,
                &object.hashed_password,
                &object.uuid,
            )
            .await;
        dbg!(ret);
        Ok(())
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

    use crate::service::user::check_unique_username_or_email;

    use super::*;

    #[test]
    fn test_username_is_lowercase() {
        let username = "MeDZik";
        let password = "password";

        let username_expected = "medzik";

        let user = User::new(username, password, false);
        assert_eq!(user.username, username_expected)
    }

    #[test]
    fn test_password_hashed() {
        let username = "username";
        let password = "password";

        let user = User::new(username, password, false);

        assert_ne!(user.hashed_password, password)
    }

    #[tokio::test]
    async fn test_insert() {
        let username = "test_username";
        let password = "test_password";

        let user = User::new(username, password, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let user_dao = UserDao::new(Extension(pool));
        let result = user_dao.insert(&user).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_complicated_username() {
        let username = "test_username";
        let password = "testpassword";

        let user = User::new(username, password, true);

        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let user_dao = UserDao::new(Extension(pool));
        let result = user_dao.insert(&user).await;
        assert!(result.is_err())
    }

    #[tokio::test]
    async fn test_get_users() {
        let db_url = "postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);

        let user_dao = UserDao::new(Extension(pool));
        // let result = user_dao.all().await;
        // assert!(result.ok())
    }

    #[tokio::test]
    async fn test_check_unique_by_username() {
        let username = "test_unique_username";

        let db_url =
            "postgresql://postgres:testpassword@192.168.50.234:5432/postgres?sslmode=disable";
        let pool = db::create_pool(&db_url);
        let user_dao = UserDao::new(Extension(pool));

        let result = user_dao.check_unique_by_username(&username).await;

        assert_eq!(result, true)
    }
}
