mod dtos;
use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
mod constant;
use constant::{PASSWORD_MAX_LENGTH, USERNAME_MAX_LENGTH};
use dtos::{CreateUserDto, LoginUserDto};
use halfmill_common::{Error, UserAction, UserWithPassword};

use crate::{app_state::AppStateWrapper, extractors::ValidatedJson};

#[axum::debug_handler]
async fn signup(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(CreateUserDto {
        username, password, ..
    }): ValidatedJson<CreateUserDto>,
) -> Result<impl IntoResponse, Error> {
    UserAction::check_if_already_exist_by_username(&state.database, username.as_bytes()).await?;
    let password = state.password_manager.hash_password(password.as_bytes())?;
    let user =
        UserAction::create(&state.database, UserWithPassword::new(username, password)).await?;
    Ok(Json(user))
}

async fn signin(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(LoginUserDto { username, password }): ValidatedJson<LoginUserDto>,
) -> Result<impl IntoResponse, Error> {
    let wrong_credentials_error = Error::BadRequest("Wrong credentials".to_string());
    if username.len() > USERNAME_MAX_LENGTH as usize
        || password.len() > PASSWORD_MAX_LENGTH as usize
    {
        return Err(wrong_credentials_error);
    }
    let user =
        UserAction::get_by_username::<UserWithPassword>(&state.database, &username)
            .await
            .map_err(|e| {
                if let Error::NotFound(_) = e {
                    wrong_credentials_error
                } else {
                    e
                }
            })?;
    let password_manager = &state.password_manager;
    password_manager.compare_password(password.as_bytes(), &user.password)?;

    Ok(Json(user))
}

pub fn auth_service() -> Router<AppStateWrapper<'static>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}
