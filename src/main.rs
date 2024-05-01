use axum::{
    routing::{patch, post},
    Router,
};
use std::sync::Arc;
use tower_http::{services::ServeFile, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod handlers;
mod models;

use handlers::{place_order, place_rider, remove_rider};

fn start() -> Router {
    let state = app::SharedState::default();

    Router::new()
        .route_service("/", ServeFile::new("index.html"))
        .route("/rider/", patch(place_rider).delete(remove_rider))
        .route("/order/", post(place_order))
        .with_state(Arc::clone(&state))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "geoprox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, start()).await.unwrap();
}
