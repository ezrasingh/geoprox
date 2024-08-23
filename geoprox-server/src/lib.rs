pub mod api;
pub mod app;
pub mod config;
pub mod dto;

mod handlers;
mod middleware;

use app::AppState;
use config::ServerConfig;
use geoprox_core::models::GeoShardConfig;

use axum::routing::{IntoMakeService, Router};
use axum::{body::Bytes, extract::Request};
use std::time::Duration;
use tower::Layer;
use tower_http::compression::CompressionLayer;
use tower_http::decompression::DecompressionLayer;
use tower_http::normalize_path::NormalizePath;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse};
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn api_service(
    server_config: ServerConfig,
    shard_config: GeoShardConfig,
) -> IntoMakeService<NormalizePath<Router>> {
    let state = AppState::new(server_config.clone(), shard_config);
    let router = Router::new()
        .nest("/api/v1/", api::routes(state))
        .merge(api::docs::router())
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                    // Add high level tracing/logging to all requests
                    TraceLayer::new_for_http()
                        .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                            tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                        })
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new()
                            .include_headers(true).
                            latency_unit(tower_http::LatencyUnit::Micros)
                        ),
                )
                // Set a timeout
                .layer(TimeoutLayer::new(
                    server_config.timeout.unwrap().into())
                )
                // Support response compression
                .layer(CompressionLayer::new())
                // Support Decompress requests
                .layer(DecompressionLayer::new())
        );

    // normalize paths on all routes
    let router = NormalizePathLayer::trim_trailing_slash().layer(router);
    axum::ServiceExt::<Request>::into_make_service(router)
}

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

    let socket: std::net::SocketAddr = server_config.clone().into();
    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, api_service(server_config, shard_config))
        .await
        .unwrap();
}
