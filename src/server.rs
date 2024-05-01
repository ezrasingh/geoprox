use crate::app;
use crate::handlers::{place_order, place_rider, remove_rider};
use axum::{
    routing::{patch, post},
    Router,
};
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tower_http::services::ServeFile;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn start() -> Router {
    let state = app::SharedState::default();
    let demo = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));

    Router::new()
        .route("/rider/", patch(place_rider).delete(remove_rider))
        .route("/order/", post(place_order))
        .nest_service("/demo", demo.clone())
        .fallback_service(demo)
        .with_state(Arc::clone(&state))
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
pub async fn run(addr: impl ToSocketAddrs) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "geoprox=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, start()).await.unwrap();
}
