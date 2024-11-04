use server::{
    dao::project::{Project, ProjectDao},
    errors::AppResult,
};
use tracing::info;
use uuid::Uuid;

#[allow(dead_code)]
pub struct TestProject {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_by: Uuid,
}

impl TestProject {
    #[allow(dead_code)]
    pub async fn create_project(pool: &db::Pool, uid: Uuid) -> AppResult<TestProject> {
        let client = pool.get().await?;
        let project_dao = ProjectDao::new(&client);
        let new_project = Project::new("测试项目".to_string(), uid, None, None);
        match project_dao.find_by_name(new_project.name.clone()).await {
            Ok(project) => Ok(TestProject {
                id: project.id,
                name: new_project.name,
                description: new_project.description,
                created_by: new_project.created_by,
            }),
            Err(_) => {
                let id = project_dao.insert(&new_project).await?;
                Ok(TestProject {
                    id,
                    name: new_project.name,
                    description: new_project.description,
                    created_by: new_project.created_by,
                })
            }
        }
    }

    pub async fn get_default_project(pool: &db::Pool, uid: Uuid) -> AppResult<TestProject> {
        let client = pool.get().await?;
        let project_dao = ProjectDao::new(&client);
        let projects = project_dao.find_projects_by_uid(uid.clone()).await?;
        info!("==============>>> projects: {projects:?}");
        let project = projects.get(0).expect("Failed to get default project");
        Ok(TestProject {
            id: project.id,
            name: project.name.clone(),
            description: project.description.clone(),
            created_by: uid,
        })
    }
}
