pub mod app;
pub mod handlers;
pub mod dto;

use crate::handlers::{place_contract, place_user, remove_user};
use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn routes() -> Router {
    let state = app::SharedState::default();

    Router::new()
        .route("/user/", post(place_user).delete(remove_user))
        .route("/contract/", post(place_contract))
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

    let root = Router::new().nest("/geoprox/", routes());
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, root).await.unwrap();
}

