mod dtos;
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
mod constant;
use dtos::CreateUser;
use halfmill_common::Database;

use crate::{app_state::AppStateWrapper, extractors::ValidatedJson};

#[axum::debug_handler]
async fn create_new_user(
    State(database): State<Database>,
    ValidatedJson(data): ValidatedJson<CreateUser>,
) -> &'static str {

    //let new_user = UserAction::create(database).await;
    "New user created"
}

async fn get_user(State(database): State<Database>) -> &'static str {
    "Me"
}

pub fn user_service() -> Router<AppStateWrapper> {
    Router::new()
        .route("/create", post(create_new_user))
        .route("/me", get(get_user))
}
