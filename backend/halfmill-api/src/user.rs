use axum::{
    routing::{get, post},
    Router,
};
use halfmill_common::UserAction;

async fn create_new_user() -> &'static str {
    //let new_user = UserAction::create(database).await;
    "New user created"
}

async fn get_user() -> &'static str {
    "Me"
}

pub fn user_service<S>() -> Router<S>
where
    S: std::clone::Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/create", post(create_new_user))
        .route("/me", get(get_user))
}
