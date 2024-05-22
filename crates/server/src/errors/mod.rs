use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use db::PoolError;
use serde::Deserialize;
use std::fmt;
use utoipa::ToSchema;

pub use tokio_postgres::Error as TokioPostgresError;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    #[error("{0} not found")]
    NotFoundError(Resource),
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, utoipa::ToSchema)]
pub struct AppResponseError {
    pub code: Option<i32>,
    pub message: String,
    pub details: Vec<(String, String)>,
}

impl AppResponseError {
    pub fn new(code: Option<i32>, message: Into<String>, details: Vec<(String, String)>) -> Self {
        Self {
            code,
            message: message.into(),
            details,
        }
    }
}

#[derive(Debug)]
pub enum CustomError {
    FaulySetup(String),
    Database(String),
}

// Allow the use of "{}" format specifier
impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomError::FaulySetup(ref cause) => write!(f, "Setup Error: {}", cause),
            CustomError::Database(ref cause) => write!(f, "Database Error: {}", cause),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
            CustomError::FaulySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };
        format!("status = {}, message = {}", status, error_message).into_response()
    }
}

impl From<axum::http::uri::InvalidUri> for CustomError {
    fn from(err: axum::http::uri::InvalidUri) -> CustomError {
        CustomError::FaulySetup(err.to_string())
    }
}

impl From<TokioPostgresError> for CustomError {
    fn from(err: TokioPostgresError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}

impl From<PoolError> for CustomError {
    fn from(err: PoolError) -> CustomError {
        CustomError::Database(err.to_string())
    }
}
