pub mod app;
pub mod handlers;
pub mod dto;

use axum::{routing, Router};
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn routes() -> Router {
    let state = app::SharedState::default();

    Router::new()
        .route("/geohash", routing::get(handlers::encode_latlng))
        .route("/geohash/:ghash", routing::get(handlers::decode_geohash))
        .route("/geohash/:ghash/neighbors", routing::get(handlers::geohash_neighbors))
        .route("/shard/:index", routing::post(handlers::create_index)
            .delete(handlers::drop_index)
            .put(handlers::insert_key)
            .get(handlers::query_range)
        )
        .with_state(Arc::clone(&state))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
pub async fn run(socket: impl ToSocketAddrs) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "geoprox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();

    let root = Router::new().nest("/api/", routes());
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, root).await.unwrap();
}

