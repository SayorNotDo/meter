use axum::{
    http::StatusCode,
    Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use strum::EnumString;
use utoipa::ToSchema;

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

pub fn invalid_input_error(field: &'static str, message: &'static str) -> AppError {
    let mut report = garde::Report::new();
    report.append(garde::Path::new(field), garde::Error::new(message));
    AppError::InvalidInputError(report)
}

pub trait ToAppResult {

}

#[derive(Debug, thiserror::Error, ToSchema)]
pub enum AppError {
    // #[error("{0} not found")]
    // NotFoundError(Resource),
    // #[error("bad request {0}")]
    // BadRequestError(String),
    #[error(transparent)]
    InvalidInputError(#[from] garde::Report),
    #[error("{0} already exists")]
    ResourceExistsError(Resource),
}

impl AppError {
    pub fn response(self) -> (StatusCode, AppResponseError) {
        use AppError::*;
        let message = self.to_string();
        let (kind, code, details, status_code) = match self {
            // NotFoundError(resource) => (
            //     format!("{resource}_NOT_FOUND_ERROR"),
            //     Some(resource.resource_type as i32),
            //     resource.details.clone(),
            //     StatusCode::NOT_FOUND,
            // ),
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

// #[derive(Debug)]
// pub enum CustomError {
//     FaultySetup(String),
//     Database(String),
// }

// Allow the use of "{}" format specifier
// impl std::fmt::Display for CustomError {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         match *self {
//             CustomError::FaultySetup(ref cause) => write!(f, "Setup Error: {}", cause),
//             CustomError::Database(ref cause) => write!(f, "Database Error: {}", cause),
//         }
//     }
// }

// impl IntoResponse for CustomError {
//     fn into_response(self) -> Response {
//         let (status, error_message) = match self {
//             CustomError::Database(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
//             CustomError::FaultySetup(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
//         };
//         format!("status = {}, message = {}", status, error_message).into_response()
//     }
// }
//
// impl From<axum::http::uri::InvalidUri> for CustomError {
//     fn from(err: axum::http::uri::InvalidUri) -> CustomError {
//         CustomError::FaultySetup(err.to_string())
//     }
// }
//
// impl From<TokioPostgresError> for CustomError {
//     fn from(err: TokioPostgresError) -> CustomError {
//         CustomError::Database(err.to_string())
//     }
// }
//
// impl From<PoolError> for CustomError {
//     fn from(err: PoolError) -> CustomError {
//         CustomError::Database(err.to_string())
//     }
// }
