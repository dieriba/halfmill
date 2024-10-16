use std::sync::Arc;

use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};

use super::constant::{PASSWORD_MAX_LENGTH, USERNAME_MAX_LENGTH};
use super::dtos::{CreateUserDto, LoginUserDto};
use halfmill_common::{
    config::{config, Config},
    constant_message::WRONG_CREDENTIALS,
    Claims, HttpError, JWTManager, UserAction, UserIdWithPassword, UserWithPassword,
};
use uuid::Uuid;

use crate::app_state::AppState;

pub async fn signup(
    state: Arc<AppState<'static>>,
    CreateUserDto {
        username, password, ..
    }: CreateUserDto,
) -> Result<impl IntoResponse, HttpError> {
    UserAction::check_if_already_exist_by_username(&state.database, username.as_bytes()).await?;
    let password = state.password_manager.hash_password(password.as_bytes())?;
    let user =
        UserAction::create(&state.database, UserWithPassword::new(username, password)).await?;
    Ok(Json(user))
}

pub async fn signin(
    AppState {
        database,
        password_manager,
        jwt_manager,
    }: &AppState<'static>,
    LoginUserDto { username, password }: LoginUserDto,
) -> Result<impl IntoResponse, HttpError> {
    let wrong_credentials_error = HttpError::bad_request(Some(WRONG_CREDENTIALS.to_string()));
    if username.len() > USERNAME_MAX_LENGTH as usize
        || password.len() > PASSWORD_MAX_LENGTH as usize
    {
        return Err(wrong_credentials_error);
    }
    // Check if user exist in database.
    let user = UserAction::get_by_username::<UserIdWithPassword>(database, &username)
        .await
        .map_err(|e| {
            if e.0 == StatusCode::NOT_FOUND {
                wrong_credentials_error
            } else {
                e
            }
        })?;

    // Check if given password match the hashed password in database
    password_manager.compare_password(password.as_bytes(), &user.password)?;
    let id = Uuid::parse_str(&user.id.to_string()).map_err(|_| {
        let id = user.id;
        tracing::error!("Invalid uuid format retrieved from database: {}", id);
        HttpError::internal_server_error()
    })?;

    //RETRIEVE EXPIRATION TIME OF ACCESS_TOKEN AND REFRESH_TOKEN FROM ENV
    let Config {
        access_token_max_age,
        refresh_token_max_age,
        ..
    } = config();
    let now = JWTManager::get_current_timestamp();
    let access_token_duration = access_token_max_age.parse::<usize>().unwrap();
    let refresh_token_duration = refresh_token_max_age.parse::<usize>().unwrap();
    let mut data = Claims::new(id, now as usize + access_token_duration);
    let acces_token = jwt_manager.get_access_token(&data)?;
    data.exp = now as usize + refresh_token_duration;
    let refresh_token = jwt_manager.get_refresh_token(&data)?;

    let access_token_cookie = format!(
        "access_token={}; HttpOnly; Max-Age={}; Path=/",
        acces_token,
        (access_token_duration)
    );

    let refresh_token_cookie = format!(
        "refresh_token={}; HttpOnly; Max-Age={}; Path=/api/auth/refresh",
        refresh_token,
        (refresh_token_duration)
    );

    // SETTING SET_COOKIE HEARDER WITH ACCESS_TOKEN AND REFRESH_TOKEN
    let set_cookie_header_value = format!("{}, {}", access_token_cookie, refresh_token_cookie);
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&set_cookie_header_value).unwrap(),
    );

    Ok((headers, Json(user)))
}
