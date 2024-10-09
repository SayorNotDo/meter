use crate::errors::{AppError, AppResult, Resource, ResourceType};
use crate::utils;
use crate::utils::time::{to_utc, to_utc_or_default};
use chrono::{DateTime, Utc};
use db::queries::project::*;
use garde::rules::AsStr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::entity::ProjectMember;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub id: i32,
    pub name: String,
    pub member_count: i32,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<String>,
    pub enable: bool,
    pub deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
    pub description: Option<String>,
    pub module_setting: Option<String>,
}

impl Project {
    #[allow(dead_code)]
    pub fn new(
        name: String,
        organization_id: i32,
        created_by: Uuid,
        description: Option<String>,
        module_setting: Option<String>,
    ) -> Self {
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
    fn to_project(&self) -> ProjectInfo;
}

macro_rules! impl_to_project {
    ($($t:ty), *) => {
        $(
        impl ToProject for $t {
            fn to_project(&self) -> ProjectInfo {
                let created_at = utils::time::to_utc(self.created_at);
                let updated_at = to_utc_or_default(self.updated_at);
                let deleted_at = to_utc_or_default(self.deleted_at);
                ProjectInfo {
                    id: self.id,
                    name: self.name.clone(),
                    member_count: self.member_count,
                    created_at,
                    created_by: self.created_by.clone(),
                    updated_at,
                    updated_by: Option::from(self.updated_by.clone()),
                    enable: self.enable,
                    deleted: self.deleted,
                    deleted_by: Option::from(self.deleted_by.clone()),
                    deleted_at,
                    description: self.description.clone(),
                    module_setting: self.module_setting.clone(),
                }
            }
        }
        )*
    };
}

impl_to_project!(FindProjectById, FindProjectsByUid);

#[derive(Debug)]
pub struct ProjectDao<'a> {
    client: &'a db::Client,
}

impl<'a> ProjectDao<'a> {
    pub fn new(client: &'a db::Client) -> Self {
        ProjectDao { client }
    }

    pub async fn check_permission_by_uid(&self, _project_id: i32, _uid: Uuid) -> AppResult<()> {
        Ok(())
    }

    pub async fn find_projects_by_uid(&self, uid: Uuid) -> AppResult<Vec<ProjectInfo>> {
        let ret = find_projects_by_uid()
            .bind(self.client, &uid)
            .all()
            .await?
            .into_iter()
            .map(|item| item.to_project())
            .collect::<Vec<_>>();
        Ok(ret)
    }

    #[allow(dead_code)]
    async fn insert(&self, object: Project) -> AppResult<i32> {
        let description = match &object.description {
            Some(s) => s.as_str(),
            None => "".as_str(),
        };
        let module_setting = match &object.module_setting {
            Some(s) => s.as_str(),
            None => "".as_str(),
        };
        let project_id = insert_project()
            .bind(
                self.client,
                &object.name.as_str(),
                &object.created_by,
                &description,
                &module_setting,
            )
            .one()
            .await?;
        Ok(project_id)
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<ProjectInfo> {
        let ret = find_project_by_id().bind(self.client, &id).opt().await?;
        match ret {
            Some(project) => Ok(project.to_project()),
            None => Err(AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: ResourceType::Project,
            })),
        }
    }

    pub async fn get_project_members(&self, id: &i32) -> AppResult<Vec<ProjectMember>> {
        let members = get_project_members()
            .bind(self.client, id)
            .all()
            .await?
            .into_iter()
            .map(|item| {
                let created_at = to_utc(item.created_at);

                ProjectMember {
                    id: item.id,
                    username: item.username,
                    email: item.email,
                    created_at,
                    last_project_id: item.last_project_id,
                }
            })
            .collect::<Vec<_>>();
        Ok(members)
    }
}
