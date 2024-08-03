pub mod api;
pub mod app;
pub mod config;
pub mod dto;
pub mod handlers;
pub mod middleware;

use config::ServerConfig;
use geoprox_core::shard::GeoShardConfig;

use axum::routing::Router;
use tower_http::trace::TraceLayer;
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

    let socket: std::net::SocketAddr = server_config.into();
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();

    let router = Router::new()
        .nest(
            "/api/v1/",
            api::routes(shard_config).layer(TraceLayer::new_for_http()),
        )
        .merge(api::docs::router());

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}
