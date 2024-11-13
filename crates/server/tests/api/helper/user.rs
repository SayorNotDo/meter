use fake::{faker::internet::en::FreeEmail, Fake, Faker};
use server::dao::permission::PermissionDao;
use server::{
    dao::{
        entity::{Permission, User},
        user::UserDao,
    },
    errors::AppResult,
    utils,
};
use std::collections::HashMap;
use strum::{EnumIter, IntoEnumIterator};
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
trait ToTestUser {
    fn to_test_user(&self) -> TestUser;
}

macro_rules! impl_to_test_user {
    ($($t:ty),*) => {
        $(
        impl ToTestUser for $t {
            fn to_test_user(&self) -> TestUser {
                TestUser {
                    id: self.id,
                    role_id: 0,
                    uuid: self.uuid,
                    email: self.email.clone(),
                    username: self.username.clone(),
                    password: "test_password".to_string(),
                    permission: vec![]
                    }
                }
            }
        )*
    };
}

impl_to_test_user!(User);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash, Clone, Copy)]
pub enum Role {
    Admin,
    User,
    System,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TestUser {
    pub id: i32,
    pub role_id: i32,
    pub uuid: Uuid,
    pub email: String,
    pub username: String,
    pub password: String,
    pub permission: Vec<Permission>,
}

impl TestUser {
    pub async fn create_user(pool: &db::Pool) -> AppResult<HashMap<Role, TestUser>> {
        let mut users = HashMap::<Role, TestUser>::new();
        let mut client = pool.get().await?;
        let transaction = client.transaction().await?;
        let user_dao = UserDao::new(&transaction);
        let perm_dao = PermissionDao::new(&transaction);
        for role in Role::iter() {
            let test_user = match role {
                Role::User => {
                    let username = Faker.fake::<String>();
                    let password: String = utils::password::generate()?;
                    let hashed_password = utils::password::hash(password.clone()).await?;
                    let email = FreeEmail().fake::<String>();
                    let user = User::new(&username, &hashed_password, &email, true);
                    let id = user_dao.insert(&user).await?;
                    let system = user_dao.find_by_username("__system__".to_string()).await?;
                    let role_id = user_dao
                        .insert_role(Faker.fake::<String>(), "PROJECT".into(), None, system.uuid)
                        .await?;
                    TestUser {
                        id,
                        role_id,
                        uuid: user.uuid,
                        email,
                        username,
                        password,
                        permission: vec![],
                    }
                }
                Role::Admin => {
                    let username = Faker.fake::<String>();
                    let password: String = utils::password::generate()?;
                    let hashed_password = utils::password::hash(password.clone()).await?;
                    let email = FreeEmail().fake::<String>();
                    let user = User::new(&username, &hashed_password, &email, true);
                    let id = user_dao.insert(&user).await?;
                    let system = user_dao.find_by_username("__system__".to_string()).await?;
                    user_dao
                        .insert_user_role_relation(user.uuid, 2, 1, system.uuid)
                        .await?;
                    let permission = perm_dao.get_permission_by_role_id(2).await?;
                    TestUser {
                        id,
                        role_id: 2,
                        uuid: user.uuid,
                        email,
                        username,
                        password,
                        permission,
                    }
                }
                Role::System => {
                    let user = user_dao.find_by_username("__system__".to_string()).await?;
                    let permission = perm_dao.get_permission_by_role_id(1).await?;
                    TestUser {
                        id: user.id,
                        role_id: 1,
                        uuid: user.uuid,
                        email: user.email,
                        username: user.username,
                        password: "test_password".to_string(),
                        permission,
                    }
                }
            };
            users.insert(role, test_user);
        }
        transaction.commit().await?;
        info!("Created users: {users:?}");
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
