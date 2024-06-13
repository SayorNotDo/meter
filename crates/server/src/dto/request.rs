use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct RegisterRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct LoginRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
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
pub struct QueryTemplateFieldParam {
    pub is_default: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_invalid_email_register_request() {
        let req = RegisterRequest::new("username", "email", "password");
        assert!(req.validate(&()).is_err());
    }
}
