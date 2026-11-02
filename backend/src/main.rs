mod error;
mod handlers;
mod health;
mod models;
mod state;
mod ws;

use crate::handlers::{check_server_now, create_server, delete_server, get_server, health as health_handler, list_servers};
use crate::health::run as health_run;
use crate::state::{SharedState, AppState};
use axum::routing::{get, post};
use axum::Router;
use std::net::SocketAddr;
use std::str::FromStr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let interval_secs: u64 = std::env::var("CHECK_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(15);
    let timeout_secs: u64 = std::env::var("CHECK_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    let state: SharedState = std::sync::Arc::new(AppState::new(Duration::from_secs(timeout_secs)));

    tokio::spawn(health_run(state.clone(), Duration::from_secs(interval_secs)));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/servers", get(list_servers).post(create_server))
        .route("/api/servers/{id}", get(get_server).delete(delete_server))
        .route("/api/servers/{id}/check", post(check_server_now))
        .route("/ws", get(ws::ws_handler))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from_str(&format!("{}:{}", host, port)).expect("invalid address");
    tracing::info!("backend listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind failed");
    axum::serve(listener, app).await.expect("server error");
}