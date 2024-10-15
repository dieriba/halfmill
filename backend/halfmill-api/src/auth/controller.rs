use super::{
    dtos::{CreateUserDto, LoginUserDto},
    service,
};
use axum::extract::State;
use axum::response::IntoResponse;
use halfmill_common::HttpError;

use crate::{app_state::AppStateWrapper, extractors::ValidatedJson};

pub async fn signup(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(create_user_dto): ValidatedJson<CreateUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    service::signup(state, create_user_dto).await
}

pub async fn signin(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    ValidatedJson(login_user_dto): ValidatedJson<LoginUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    service::signin(&state, login_user_dto).await
}
