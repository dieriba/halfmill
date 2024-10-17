use axum::response::IntoResponse;
use halfmill_common::HttpError;
use halfmill_worker::jobs;

use super::dtos::ScriptDto;

pub async fn run_script(
    ScriptDto { language, code }: ScriptDto,
) -> Result<impl IntoResponse, HttpError> {
    tracing::info!(language, code);
    jobs::run_script(code).await
}
