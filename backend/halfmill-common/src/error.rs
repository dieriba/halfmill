use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal Server Error: {0}")]
    InternalErr(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    AlreadyExists(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::AlreadyExists(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(SingleErrorResponse::new(self.to_string()))).into_response()
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorsResponse {
    message: Vec<String>,
}

impl ErrorsResponse {
    pub fn new(error_messages: Vec<String>) -> Self {
        Self {
            message: error_messages,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct SingleErrorResponse {
    message: String,
}

impl SingleErrorResponse {
    pub fn new(error_message: String) -> Self {
        Self {
            message: error_message,
        }
    }
}
