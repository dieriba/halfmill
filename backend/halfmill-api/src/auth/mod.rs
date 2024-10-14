mod dtos;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
mod constant;
use dtos::CreateUserDto;
use halfmill_common::Database;

use crate::{app_state::AppStateWrapper, extractors::ValidatedJson};

#[axum::debug_handler]
async fn signup(
    State(database): State<Database>,
    ValidatedJson(data): ValidatedJson<CreateUserDto>,
) -> &'static str {
    //let new_user = UserAction::create(database).await;
    "New user created"
}

async fn signin(State(database): State<Database>) -> &'static str {
    "Me"
}

pub fn auth_service() -> Router<AppStateWrapper> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", get(signin))
}
