use axum::{routing::post, Router};

use crate::app_state::AppStateWrapper;

mod controller;
mod service;

pub(in crate::auth) mod constant;
pub(super) mod dtos;

pub fn auth_service() -> Router<AppStateWrapper<'static>> {
    Router::new()
        .route("/signup", post(controller::signup))
        .route("/signin", post(controller::signin))
}
