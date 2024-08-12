pub mod api;
pub mod app;
pub mod config;
pub mod dto;
pub mod handlers;
pub mod middleware;

use app::AppState;
use config::ServerConfig;
use geoprox_core::models::GeoShardConfig;

use axum::extract::Request;
use axum::routing::Router;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Start http server
#[tokio::main]
pub async fn run(server_config: ServerConfig, shard_config: GeoShardConfig) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "geoprox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState::new(server_config.clone(), shard_config);
    let router = Router::new()
        .nest("/api/v1/", api::routes(state))
        .merge(api::docs::router());

    // normalize paths on all routes
    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let socket: std::net::SocketAddr = server_config.into();
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(
        listener,
        axum::ServiceExt::<Request>::into_make_service(router),
    )
    .await
    .unwrap();
}
