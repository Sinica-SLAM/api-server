use anyhow;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::io;
use thiserror::Error;

pub type Result<T, E = HttpError> = core::result::Result<T, E>;

#[derive(Debug, Serialize)]
pub struct ErrorInfo {
    message: String,
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[allow(dead_code)]
    #[error("Not Found: {0}")]
    NotFound(String),

    #[allow(dead_code)]
    #[error("Job Running")]
    Conflict,

    #[allow(dead_code)]
    #[error("Job Failed")]
    ExpectationFailed,

    #[allow(dead_code)]
    #[error("Invalid params: {0:?}")]
    InvalidParams(Vec<&'static str>),

    #[allow(dead_code)]
    #[error("Invalid file format")]
    InvalidFileFormat,

    #[allow(dead_code)]
    #[error("Error parsing `multipart/form-data` request.\n{0}")]
    MultipartError(String),

    #[allow(dead_code)]
    #[error(transparent)]
    IOError(#[from] io::Error),

    #[allow(dead_code)]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            HttpError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            HttpError::Conflict => (StatusCode::CONFLICT, self.to_string()),
            HttpError::InvalidParams(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            HttpError::MultipartError(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            HttpError::InvalidFileFormat => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            HttpError::IOError(_) => (StatusCode::NOT_FOUND, self.to_string()),
            HttpError::Other(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            HttpError::ExpectationFailed => (StatusCode::EXPECTATION_FAILED, self.to_string()),
        };
        let body = Json(ErrorInfo { message: err_msg });
        (status, body).into_response()
    }
}
