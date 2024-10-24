pub(crate) const POOL_CONNECTIONS: u32 = 10;

pub mod config;
mod database;
mod error;
mod scripts;
mod utils;

use axum::{http::StatusCode, response::IntoResponse, Json};
pub use database::*;
pub use error::*;
pub use scripts::*;
use serde::Serialize;
pub use utils::*;
#[derive(Serialize)]
pub struct ServerResponse<T>
where
    T: Serialize,
{
    data: T,
    message: String,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

impl<T> ServerResponse<T>
where
    T: Serialize,
{
    pub fn new(data: T, message: String, status_code: Option<StatusCode>) -> ServerResponse<T> {
        Self {
            data,
            message,
            status_code: status_code.unwrap_or(StatusCode::OK),
        }
    }
}

impl<T> IntoResponse for ServerResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (self.status_code, Json(self)).into_response()
    }
}
