use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}
pub mod constant_message {
    pub static WRONG_CREDENTIALS: &str = "Wrong Credentials";
}

#[derive(Debug)]
pub struct HttpError(pub StatusCode, String);

impl HttpError {
    pub fn resource_not_found(error_message: Option<String>) -> HttpError {
        Self(
            StatusCode::NOT_FOUND,
            error_message.unwrap_or("ressource not found".to_string()),
        )
    }

    pub fn bad_request(error_message: Option<String>) -> HttpError {
        Self(
            StatusCode::BAD_REQUEST,
            error_message.unwrap_or("bad_request".to_string()),
        )
    }

    pub fn unauthorized() -> HttpError {
        Self(StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
    }

    pub fn forbidden() -> HttpError {
        Self(StatusCode::FORBIDDEN, "forbidden".to_string())
    }

    pub fn unprocessable_entity(error_message: Option<String>) -> HttpError {
        Self(
            StatusCode::UNPROCESSABLE_ENTITY,
            error_message.unwrap_or("unprocessable_entity".to_string()),
        )
    }

    pub fn internal_server_error() -> HttpError {
        Self(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".to_string(),
        )
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (self.0, Json(SingleErrorResponse::new(self.1))).into_response()
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
