use crate::dao::entity::{CustomField, Step};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    #[garde(length(min = 30))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ProjectQueryParam {
    pub organization_id: i32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct QueryTemplateParam {
    pub is_default: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct ListQueryParam {
    pub module_id: Option<i32>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct CaseQueryParam {
    pub is_deleted: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateElementRequest {
    pub name: String,
    pub value: String,
    pub element_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateFunctionalCaseRequest {
    pub name: String,
    pub module_id: i32,
    pub template_id: i32,
    pub tags: Option<String>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IssueRelationRequest {
    pub case_id: i32,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub issue_id: String,
    pub source: String,
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateScriptRequest {
    pub case_id: i32,
    pub name: String,
    pub environment: String,
    pub pre_processors: Vec<Step>,
    pub steps: Vec<Step>,
    pub after_processors: Vec<Step>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DiagnoseRequest {
    pub machine_id: i32,
    pub script_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_invalid_email_register_request() {
        let req = RegisterRequest {
            username: "username".into(),
            email: "email".into(),
            password: "password".into(),
        };
        assert!(req.validate(&()).is_err());
    }
}
