mod dtos;
use axum::{extract::State, routing::get, Router};
use halfmill_common::Database;

use crate::app_state::AppStateWrapper;

async fn get_user(State(database): State<Database>) -> &'static str {
    "Me"
}

pub fn user_service() -> Router<AppStateWrapper<'static>> {
    Router::new().route("/me", get(get_user))
}
