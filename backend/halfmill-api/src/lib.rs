use anyhow::Result;
use app_state::init_app_state;
use axum::{
    http::{self, HeaderValue, Method},
    Router,
};
mod auth;
use auth::auth_service;
use halfmill_common::{
    config::{config, Config},
    Database,
};
use script::script_service;
use tokio::sync::oneshot::Receiver;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use users::user_service;

mod app_state;
mod extractors;
mod middleware;
mod script;
mod users;

pub async fn start_server(database: Database, rx: Receiver<()>) -> Result<()> {
    let Config { backend_port, .. } = config();
    let port = backend_port.as_str();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config().backend_port))
        .await
        .unwrap();
    let origin = format!("http://localhost:{}", backend_port);
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(origin.parse::<HeaderValue>().unwrap())
        .allow_credentials(true);
    let services = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    let app_state = init_app_state(database);

    let app = Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/auth", auth_service())
                .nest(
                    "/user",
                    user_service().layer(axum::middleware::from_fn_with_state(
                        app_state.clone(),
                        middleware::authenticate_middleware,
                    )),
                )
                .nest(
                    "/script",
                    script_service().layer(axum::middleware::from_fn_with_state(
                        app_state.clone(),
                        middleware::authenticate_middleware,
                    )),
                ),
        )
        .layer(ServiceBuilder::new().layer(services))
        .with_state(app_state)
        .fallback(|| async { (http::StatusCode::NOT_FOUND, "Ressource Not found") });

    let server = axum::serve(listener, app);

    let graceful = server.with_graceful_shutdown(async move {
        tracing::info!("Server is now listening on port: {port}");
        rx.await.ok();
        tracing::info!("Server gracefully shutdown!!");
    });

    if let Err(e) = graceful.await {
        tracing::error!("server error: {}", e);
    }

    Ok(())
}
