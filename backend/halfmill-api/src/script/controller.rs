use axum::response::IntoResponse;
use halfmill_common::HttpError;

use crate::extractors::ValidatedJson;

use super::{dtos::ScriptDto, services};

pub async fn run_script(
    ValidatedJson(script): ValidatedJson<ScriptDto>,
) -> Result<impl IntoResponse, HttpError> {
    services::run_script(script).await
}
