mod dtos;
use axum::{routing::get, Router};
mod controller;
mod services;
use crate::app_state::AppStateWrapper;

pub fn user_service() -> Router<AppStateWrapper<'static>> {
    Router::new().route("/me", get(controller::get_user))
}
