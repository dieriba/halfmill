mod app_state;
mod jobs;
mod script;
mod user;

use anyhow::Result;
use app_state::init_app_state;
use axum::{
    http::{self, Method},
    Router,
};
use halfmill_common::{config::config, Database};
use tokio::sync::oneshot::Receiver;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use user::user_service;

pub async fn start_server(database: Database, rx: Receiver<()>) -> Result<()> {
    let port = &config().backend_port;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config().backend_port))
        .await
        .unwrap();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let services = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let app = Router::new()
        .nest("/api", Router::new().nest("/user", user_service()))
        .layer(ServiceBuilder::new().layer(services))
        .with_state(init_app_state(database))
        .fallback(|| async { (http::StatusCode::NOT_FOUND, "Ressource Not found") });

    let server = axum::serve(listener, app);

    let graceful = server.with_graceful_shutdown(async move {
        tracing::info!("Server is listening on port: {port}");
        rx.await.ok();
    });

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
