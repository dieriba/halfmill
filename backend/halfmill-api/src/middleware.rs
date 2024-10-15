use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use halfmill_common::{HttpError, UserAction, UserId};

use crate::app_state::{AppState, AppStateWrapper};

pub async fn authenticate_middleware(
    State(AppStateWrapper(state)): State<AppStateWrapper<'static>>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, HttpError> {
    tracing::info!("Entered inside authenticate middleware");
    let access_token = jar
        .get("access_token")
        .map(|cookie| cookie.value())
        .ok_or_else(HttpError::unauthorized)?;
    tracing::info!("access token value: {}", access_token);
    let AppState {
        database,
        jwt_manager,
        ..
    } = &*state;

    let user_id = jwt_manager
        .validate_access_token(access_token)?
        .id
        .to_string();
    tracing::info!("Token validated");
    tracing::info!("Querying user based on id: {}", user_id);
    let user = UserAction::get_by_id::<UserId>(database, user_id.as_str())
        .await
        .map_err(|_| HttpError::unauthorized())?;

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
