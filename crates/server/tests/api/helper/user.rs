use fake::{faker::internet::en::FreeEmail, Fake, Faker};
use server::{
    dao::{entity::User, user::UserDao},
    errors::AppResult,
    utils,
};
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};
use tracing::info;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash)]
pub enum Role {
    Admin,
    User,
    System,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestUser {
    pub id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub async fn create_user(pool: &db::Pool) -> AppResult<HashMap<Role, TestUser>> {
        let mut users = HashMap::<Role, TestUser>::new();
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        let user_dao = UserDao::new(&transaction);
        for role in Role::iter() {
            let username = Faker.fake::<String>();
            let password: String = utils::password::generate()?;
            let hashed_password = utils::password::hash(password.clone()).await?;
            let email = FreeEmail().fake::<String>();
            let user = User::new(&username, &hashed_password, &email, true);
            let id = user_dao.insert(&user).await?;
            let test_user = TestUser {
                id,
                uuid: user.uuid,
                email,
                username,
                password,
            };
            users.insert(role, test_user);
        }
        transaction.commit().await?;
        info!("logging created users: {users:?}");
        Ok(users)
    }

    pub async fn disable_user(pool: &db::Pool, id: i32) -> AppResult {
        let client = pool.get().await?;
        let user_dao = UserDao::new(&client);
        user_dao.batch_update_user_status(false, vec![id]).await?;
        Ok(())
    }

    pub async fn enable_user(pool: &db::Pool, id: i32) -> AppResult {
        let client = pool.get().await?;
        let user_dao = UserDao::new(&client);
        user_dao.batch_update_user_status(true, vec![id]).await?;
        Ok(())
    }
}
