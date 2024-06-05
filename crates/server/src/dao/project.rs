use chrono::{DateTime, Utc};
use garde::rules::AsStr;
use serde::{Deserialize, Serialize};
use db::queries::project::*;
use tokio_postgres::error::DbError;
use uuid::Uuid;
use crate::dao::base::BaseDao;
use crate::errors::{AppResult, AppError, Resource, ResourceType};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub organization_id: i32,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<Uuid>,
    pub deleted_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub module_setting: Option<String>,
}

impl Project {
    #[allow(dead_code)]
    pub fn new(name: String,
               organization_id: i32,
               created_by: Uuid,
               description: Option<String>,
               module_setting: Option<String>) -> Self {
        Self {
            id: 0,
            name,
            organization_id,
            created_at: Utc::now(),
            created_by,
            updated_at: None,
            updated_by: None,
            deleted_by: None,
            deleted_at: None,
            description,
            module_setting,
        }
    }
}

trait ToProject {
    fn to_project(&self) -> Project;
}

macro_rules! impl_to_project {
    ($($t:ty), *) => {
        $(
        impl ToProject for $t {
            fn to_project(&self) -> Project {
                let timestamp_created_at = self.created_at.assume_utc().unix_timestamp_nanos();
                let timestamp_updated_at = match self.updated_at {
                    Some(t) => t.assume_utc().unix_timestamp_nanos(),
                    None => 0
                };
                let timestamp_deleted_at = match self.deleted_at {
                    Some(t) => t.assume_utc().unix_timestamp_nanos(),
                    None => 0
                };
                Project {
                    id: self.id,
                    name: self.name.clone(),
                    organization_id: self.organization_id,
                    created_at: DateTime::from_timestamp_nanos(timestamp_created_at as i64),
                    created_by: self.created_by,
                    updated_at: Option::from(DateTime::from_timestamp_nanos(timestamp_updated_at as i64)),
                    updated_by: Option::from(self.updated_by),
                    deleted_by: Option::from(self.deleted_by),
                    deleted_at: Option::from(DateTime::from_timestamp_nanos(timestamp_deleted_at as i64)),
                    description: self.description.clone(),
                    module_setting: self.module_setting.clone(),
                }
            }
        }
        )*
    };
}

impl_to_project!(FindProjectById);

#[derive(Debug)]
pub struct ProjectDao {
    client: db::Client,
}

impl ProjectDao {
    pub fn new(client: db::Client) -> Self {
        ProjectDao { client }
    }

    pub async fn check_permission_by_uid(&self, _project_id: i32, _uid: Uuid) -> AppResult<()> {
        Ok(())
    }

    pub async fn find_projects_by_uid(&self, _uid: Uuid) -> AppResult<Vec<Project>> {
        Ok(vec![])
    }
}

impl BaseDao<Project> for ProjectDao {
    async fn all(&self) -> AppResult<Vec<Project>> {
        todo!()
    }

    async fn insert(&self, object: Project) -> AppResult<i32> {
        let description = match &object.description {
            Some(s) => s.as_str(),
            None => "".as_str()
        };
        let module_setting = match &object.module_setting {
            Some(s) => s.as_str(),
            None => "".as_str()
        };
        let project_id = insert_project()
            .bind(
                &self.client,
                &object.name.as_str(),
                &object.organization_id,
                &object.created_by,
                &description,
                &module_setting
            )
            .one()
            .await?;
        Ok(project_id)
    }

    async fn find_by_id(&self, id: i32) -> AppResult<Project> {
        let ret = find_project_by_id()
            .bind(&self.client, &id)
            .opt()
            .await?;
        match ret {
            Some(project) => Ok(project.to_project()),
            None => {
                Err(AppError::NotFoundError(Resource {
                    details: vec![],
                    resource_type: ResourceType::Project,
                }))
            }
        }
    }

    async fn update(&self, _object: &Project) -> Result<Project, DbError> {
        todo!()
    }

    async fn delete_by_id(&self, _id: i32) -> AppResult<()> {
        todo!()
    }
}
