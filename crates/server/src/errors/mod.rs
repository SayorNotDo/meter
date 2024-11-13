pub mod custom_extractor;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use lettre::transport::smtp::Error as SmtpError;
use serde::{Deserialize, Serialize};
use ssh2::Error as SshError;
use strum::EnumString;
use tera::Error as TeraError;
use tokio_postgres::Error as TokioPostgresError;
use utoipa::ToSchema;

use crate::dao::entity;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ToSchema)]
pub struct Resource {
    pub details: Vec<(String, String)>,
    pub resource_type: ResourceType,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        self.resource_type.fmt(f)
    }
}

#[derive(
    Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ToSchema,
)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "FILE")]
    File,
    #[strum(serialize = "SESSION")]
    Session,
    #[strum(serialize = "MESSAGE")]
    Message,
    #[strum(serialize = "PROJECT")]
    Project,
    #[strum(serialize = "ROLE")]
    Role,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("{0} not found")]
    NotFoundError(Resource),
    #[error("{0} not available")]
    NotAvailableError(Resource),
    #[error("{0} already exists")]
    ResourceExistsError(Resource),
    #[error("bad request: {0}")]
    BadRequestError(String),
    #[error("forbidden: {0}")]
    ForbiddenError(String),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error("{0}")]
    InvalidPayloadError(String),
    #[error("{0}")]
    InvalidSessionError(String),
    #[error(transparent)]
    InvalidInputError(#[from] garde::Report),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    HashError(String),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    JsonExtractRejection(#[from] JsonRejection),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    HttpClientError(#[from] reqwest::Error),
    #[error(transparent)]
    DatabaseError(#[from] TokioPostgresError),
    #[error(transparent)]
    SpawnTaskError(#[from] tokio::task::JoinError),
    #[error("{0} convert error")]
    TimeConvertError(Resource),
    #[error(transparent)]
    TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
    #[error(transparent)]
    ExtensionRejectionError(#[from] axum::extract::rejection::ExtensionRejection),
    #[error(transparent)]
    DbPoolError(#[from] db::PoolError),
    #[error("{0}")]
    UnauthorizedError(String),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    TeraError(#[from] TeraError),
    #[error(transparent)]
    SshError(#[from] SshError),
    #[error(transparent)]
    SmtpError(#[from] SmtpError),
    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),
}

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
    let mut report = garde::Report::new();
    report.append(garde::Path::new(field), garde::Error::new(message));
    AppError::InvalidInputError(report)
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::HashError(value.to_string())
    }
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        use AppError::*;
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {
            NotFoundError(resource) => (
                format!("{resource}_NOT_FOUND_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::NOT_FOUND,
            ),
            ForbiddenError(_err) => (
                "FORBIDDEN_ERROR".to_string(),
                None,
                vec![],
                StatusCode::FORBIDDEN,
            ),
            NotAvailableError(resource) => (
                format!("{resource}_NOT_AVAILABLE_ERROR"),
                None,
                vec![],
                StatusCode::NOT_FOUND,
            ),
            InvalidSessionError(_err) => (
                "INVALID_SESSION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            ResourceExistsError(resource) => (
                format!("{resource}_ALREADY_EXISTS_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::CONFLICT,
            ),
            InvalidPayloadError(_err) => (
                "INVALID_PAYLOAD_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            TimeConvertError(_err) => (
                "TIME_CONVERT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            ConfigError(_) => (
                "CONFIG_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AddrParseError(_err) => (
                "ADDR_PARSE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            HashError(_err) => (
                "HASH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            RedisError(_err) => (
                "REDIS_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            ParseJsonError(_err) => (
                "PARSE_JSON_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            JsonExtractRejection(err) => (
                "JSON_EXTRACT_REJECTION".to_string(),
                None,
                vec![("err_msg".to_string(), err.body_text())],
                err.status(),
            ),
            IoError(err) => {
                let (status, kind, code) = match err.kind() {
                    std::io::ErrorKind::NotFound => (
                        StatusCode::NOT_FOUND,
                        format!("{}_NOT_FOUND_ERROR", ResourceType::File),
                        Some(ResourceType::File as i32),
                    ),
                    std::io::ErrorKind::PermissionDenied => {
                        (StatusCode::FORBIDDEN, "FORBIDDEN_ERROR".to_string(), None)
                    }
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "IO_ERROR".to_string(),
                        None,
                    ),
                };
                (kind, code, vec![], status)
            }
            JwtError(_err) => (
                "UNAUTHORIZED_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            HttpClientError(_err) => (
                "HTTP_CLIENT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            BadRequestError(_err) => (
                "BAD_REQUEST_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            InvalidInputError(err) => (
                "INVALID_INPUT_ERROR".to_string(),
                None,
                err.iter()
                    .map(|(p, e)| (p.to_string(), e.to_string()))
                    .collect(),
                StatusCode::BAD_REQUEST,
            ),
            DatabaseError(_err) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            DbPoolError(_err) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            SpawnTaskError(_err) => (
                "SPAWN_TASK_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            TypeHeaderError(_err) => (
                "TYPE_HEADER_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            ExtensionRejectionError(_err) => (
                "EXTENSION_REJECTION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            AxumError(_err) => (
                "AXUM_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            UnauthorizedError(_err) => (
                "UNAUTHORIZED_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            UnknownError(_err) => (
                "UNKNOWN_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            Infallible(_err) => (
                "INFALLIBLE".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            TeraError(_err) => (
                "TERA_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            SshError(_err) => (
                "SSH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            SmtpError(_err) => (
                "SMTP_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            LettreError(_err) => (
                "LETTRE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        (
            status_code,
            AppResponseError::new(kind, message, code, details),
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.response();
        (status_code, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
    pub kind: String,
    pub message: String,
    pub code: Option<i32>,
    pub details: Vec<(String, String)>,
}

impl AppResponseError {
    pub fn new(
        kind: impl Into<String>,
        message: impl Into<String>,
        code: Option<i32>,
        details: Vec<(String, String)>,
    ) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
            code,
            details,
        }
    }
}

pub trait ToAppResult {
    type Output: entity::AppEntity;

    fn to_result(self) -> AppResult<Self::Output>;
    fn check_absent(self) -> AppResult;
    fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult;
    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T>
where
    T: entity::AppEntity,
{
    type Output = T;
    fn to_result(self) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            })
        })
    }

    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            })
        })
    }

    fn check_absent(self) -> AppResult {
        if self.is_some() {
            Err(AppError::ResourceExistsError(Resource {
                details: vec![],
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }

    fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult {
        if self.is_some() {
            Err(AppError::ResourceExistsError(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            }))
        } else {
            Ok(())
        }
    }
}
