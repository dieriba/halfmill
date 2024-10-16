use axum::{routing::post, Router};
mod controller;
mod services;
mod dtos;
use crate::app_state::AppStateWrapper;

pub fn script_service() -> Router<AppStateWrapper<'static>> {
    Router::new().route("/run", post(controller::run_script))
}
