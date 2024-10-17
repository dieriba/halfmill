use axum::{http::StatusCode, response::IntoResponse, Json};
use halfmill_common::HttpError;
use tokio::{fs::File, io::AsyncWriteExt, process::Command};

use chrono::Utc;

use crate::ScriptOutput;
pub async fn run_script(code: String) -> Result<impl IntoResponse, HttpError> {
    let path = format!("/tmp/half-mill/python/{}.py", Utc::now().timestamp());
    let mut file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&path)
        .await
        .map_err(|_| HttpError::internal_server_error())?;
    file.write_all(code.as_bytes())
        .await
        .map_err(|_| HttpError::internal_server_error())?;
    let output = Command::new("python3")
        .arg(path)
        .output()
        .await
        .map_err(|_| HttpError::internal_server_error())?;

    let response = String::from_utf8({
        if !output.stdout.is_empty() {
            output.stdout
        } else {
            output.stderr
        }
    })
    .map_err(|_| HttpError::internal_server_error())?;
    let response = ScriptOutput::new(response);
    Ok((StatusCode::OK, Json(response)))
}
