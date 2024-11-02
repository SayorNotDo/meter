use server::{
    dao::project::{Project, ProjectDao},
    errors::AppResult,
};
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestProject {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

impl TestProject {
    pub async fn create_project(pool: &db::Pool, uid: Uuid) -> AppResult<TestProject> {
        let client = pool.get().await?;
        let project_dao = ProjectDao::new(&client);
        let new_project = Project::new("测试项目".to_string(), uid, None, None);
        let project = project_dao.insert(&new_project).await?;
        Ok(TestProject {
            id: project,
            name: new_project.name,
            description: new_project.description,
            created_by: new_project.created_by,
        })
    }
}
