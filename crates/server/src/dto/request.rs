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


#[allow(dead_code)]
impl RegisterRequest {
    pub fn new(username: &str, password: &str, email: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
            email: email.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct LoginRequest {
    #[garde(ascii, length(min = 3, max = 25))]
    pub username: String,
    #[garde(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
pub struct RefreshTokenRequest {
    #[garde(length(min = 30))]
    pub token: String,
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
