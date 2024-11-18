use fake::{faker::internet::en::FreeEmail, Fake, Faker};
use server::dao::permission::PermissionDao;
use server::service::user::batch_delete;
use server::state::AppState;
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
    DeletedUser,
}

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
                    let permission_list = vec![1];
                    let role_id = create_role(permission_list, &user_dao, &perm_dao).await?;
                    let user = create_user_with_role(role_id, &user_dao, &perm_dao).await?;
                    user
                }
                Role::DeletedUser => {
                    let permission_list = perm_dao
                        .get_permission_by_role_id(1)
                        .await?
                        .into_iter()
                        .map(|item| item.id)
                        .collect::<_>();
                    let role_id = create_role(permission_list, &user_dao, &perm_dao).await?;
                    let user = create_user_with_role(role_id, &user_dao, &perm_dao).await?;
                    let deleted_by = user_dao.find_by_username("__system__".to_string()).await?;
                    user_dao
                        .soft_deleted_user(deleted_by.uuid, &user.id)
                        .await?;
                    user
                }
                Role::Admin => {
                    let user = create_user_with_role(2, &user_dao, &perm_dao).await?;
                    user
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

    pub async fn delete_user(state: &AppState, uids: Vec<i32>) -> AppResult {
        let client = state.pool.get().await?;
        let user_dao = UserDao::new(&client);
        let created_by = user_dao.find_by_username("__system__".to_string()).await?;
        batch_delete(state, created_by.uuid, uids).await?;
        Ok(())
    }
}

async fn create_role<T>(
    permission_list: Vec<i32>,
    user_dao: &UserDao<'_, T>,
    perm_dao: &PermissionDao<'_, T>,
) -> AppResult<i32>
where
    T: db::GenericClient,
{
    let created_by = user_dao.find_by_username("__system__".to_string()).await?;
    let role_id = user_dao
        .insert_role(
            Faker.fake::<String>(),
            "PROJECT".into(),
            None,
            created_by.uuid,
        )
        .await?;
    perm_dao
        .insert_role_permission_relation(role_id, permission_list)
        .await?;
    Ok(role_id)
}

async fn create_user_with_role<T>(
    role_id: i32,
    user_dao: &UserDao<'_, T>,
    perm_dao: &PermissionDao<'_, T>,
) -> AppResult<TestUser>
where
    T: db::GenericClient,
{
    let username = Faker.fake::<String>();
    let password: String = utils::password::generate()?;
    let email = FreeEmail().fake::<String>();
    let hashed_password = utils::password::hash(password.clone()).await?;
    let user = User::new(&username, &hashed_password, &email, true);
    let id = user_dao.insert(&user).await?;
    let permission = perm_dao.get_permission_by_role_id(role_id).await?;
    let created_by = user_dao.find_by_username("__system__".to_string()).await?;
    user_dao
        .insert_user_role_relation(user.uuid, role_id, 1, created_by.uuid)
        .await?;
    Ok(TestUser {
        id,
        role_id,
        uuid: user.uuid,
        username,
        email,
        password,
        permission,
    })
}
