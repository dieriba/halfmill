use axum::{http::StatusCode, response::IntoResponse};
use halfmill_common::HttpError;

use super::dtos::ScriptDto;

pub async fn run_script(
    ScriptDto { language, content }: ScriptDto,
) -> Result<impl IntoResponse, HttpError> {
    tracing::info!(language, content);
    Ok(StatusCode::OK)
}
