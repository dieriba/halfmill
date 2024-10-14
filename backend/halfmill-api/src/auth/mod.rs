mod dtos;
use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
mod constant;
use dtos::CreateUserDto;
use halfmill_common::{CreateUser, Database, Error, UserAction};

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
    let user = UserAction::create(&state.database, CreateUser::new(username, password)).await?;
    Ok(Json(user))
}

async fn signin(State(database): State<Database>) -> &'static str {
    "Me"
}

pub fn auth_service() -> Router<AppStateWrapper<'static>> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", get(signin))
}
