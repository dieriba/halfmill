use axum::{extract::State, response::IntoResponse, Extension};
use halfmill_common::{Database, HttpError, UserId};

use super::services;

pub async fn get_user(
    Extension(user): Extension<UserId>,
    State(database): State<Database>,
) -> Result<impl IntoResponse, HttpError> {
    services::get_user(&database, user).await
}
