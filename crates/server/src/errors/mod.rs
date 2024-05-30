use axum::{
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use tokio_postgres::Error as TokioPostgresError;
use utoipa::ToSchema;

use crate::dao;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, EnumString, strum::Display, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "FILE")]
    File,
    #[strum(serialize = "SESSION")]
    Session,
    #[strum(serialize = "MESSAGE")]
    Message,
}


pub trait ToAppResult {
    type Output: dao::Entity;
    fn to_result(self) -> AppResult<Self::Output>;
    fn check_absent(self) -> AppResult;
    fn check_absent_details(self, details: Vec<(String, String)>) -> AppResult;
    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output>;
}

impl<T> ToAppResult for Option<T>
    where
        T: dao::Entity,
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
    fn to_result_details(self, details: Vec<(String, String)>) -> AppResult<Self::Output> {
        self.ok_or_else(|| {
            AppError::NotFoundError(Resource {
                details,
                resource_type: Self::Output::RESOURCE,
            })
        })
    }
}

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    #[error("{0} not found")]
    NotFoundError(Resource),
    // #[error("bad request {0}")]
    // BadRequestError(String),
    #[error("{0}")]
    InvalidSessionError(String),
    #[error(transparent)]
    InvalidInputError(#[from] garde::Report),
    #[error("{0}")]
    HashError(String),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("{0} already exists")]
    ResourceExistsError(Resource),
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
            InvalidSessionError(_err) => (
                "INVALID_SESSION_ERROR".to_string(),
                None,
                vec![],
                StatusCode::BAD_REQUEST,
            ),
            TimeConvertError(_err) => (
                "TIME_CONVERT_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR
            ),
            HashError(_err) => (
                "HASH_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR
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
            JwtError(_err) => (
                "UNAUTHORIZED_ERROR".to_string(),
                None,
                vec![],
                StatusCode::UNAUTHORIZED,
            ),
            // BadRequestError(_err) => (
            //     "BAD_REQUEST_ERROR".to_string(),
            //     None,
            //     vec![],
            //     StatusCode::BAD_REQUEST,
            // ),
            InvalidInputError(err) => (
                "INVALID_INPUT_ERROR".to_string(),
                None,
                err.iter()
                    .map(|(p, e)| (p.to_string(), e.to_string()))
                    .collect(),
                StatusCode::BAD_REQUEST,
            ),
            ResourceExistsError(resource) => (
                format!("{resource}_ALREADY_EXISTS_ERROR"),
                Some(resource.resource_type as i32),
                resource.details.clone(),
                StatusCode::CONFLICT,
            ),
            DatabaseError(_err) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR
            ),
            DbPoolError(_err) => (
                "DATABASE_ERROR".to_string(),
                None,
                vec![],
                StatusCode::INTERNAL_SERVER_ERROR
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
                StatusCode::INTERNAL_SERVER_ERROR
            )
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
