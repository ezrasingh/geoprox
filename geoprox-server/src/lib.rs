pub mod api;
pub mod app;
pub mod dto;
pub mod handlers;

use axum::routing::Router;
use geoprox_core::shard::GeoShardConfig;
use tokio::net::ToSocketAddrs;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
pub async fn run(socket: impl ToSocketAddrs, shard_config: Option<GeoShardConfig>) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "geoprox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
