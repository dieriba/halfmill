use axum::{response::IntoResponse, Json};
use halfmill_common::{Database, HttpError, User, UserAction, UserId};

pub async fn get_user(database: &Database, user: UserId) -> Result<impl IntoResponse, HttpError> {
    let user = UserAction::get_by_id::<User>(database, &user.id.to_string()).await?;

    Ok(Json(user))
}
