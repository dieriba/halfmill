mod dtos;
use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
mod constant;
use constant::{PASSWORD_MAX_LENGTH, USERNAME_MAX_LENGTH};
use dtos::{CreateUserDto, LoginUserDto};
use halfmill_common::{
    constant_message::WRONG_CREDENTIALS, HttpError, UserAction, UserId, UserIdWithPassword,
    UserWithPassword,
};
use uuid::Uuid;

use crate::{
    app_state::{AppState, AppStateWrapper},
    extractors::ValidatedJson,
};

#[axum::debug_handler]
async fn signup(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(CreateUserDto {
        username, password, ..
    }): ValidatedJson<CreateUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    UserAction::check_if_already_exist_by_username(&state.database, username.as_bytes()).await?;
    let password = state.password_manager.hash_password(password.as_bytes())?;
    let user =
        UserAction::create(&state.database, UserWithPassword::new(username, password)).await?;
    Ok(Json(user))
}

async fn signin(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(LoginUserDto { username, password }): ValidatedJson<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    let wrong_credentials_error = HttpError::bad_request(Some(WRONG_CREDENTIALS.to_string()));
    let AppState {
        database,
        password_manager,
        jwt_manager,
    } = &*state;
    if username.len() > USERNAME_MAX_LENGTH as usize
        || password.len() > PASSWORD_MAX_LENGTH as usize
    {
        return Err(wrong_credentials_error);
    }
    let user = UserAction::get_by_username::<UserIdWithPassword>(database, &username)
        .await
        .map_err(|e| {
            if e.0 == StatusCode::NOT_FOUND {
                wrong_credentials_error
            } else {
                e
            }
        })?;
    password_manager.compare_password(password.as_bytes(), &user.password)?;
    let id =
        Uuid::parse_str(&user.id.to_string()).map_err(|_| {
            let id = user.id;
            tracing::error!("Invalid uuid format retrieved from database: {}", id);
            HttpError::internal_server_error()
        })?;
    let token = jwt_manager.get_access_token(UserId { id })?;
    tracing::info!("jwt token: {}", token);
    Ok(Json(user))
}

pub fn auth_service() -> Router<AppStateWrapper<'static>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}
