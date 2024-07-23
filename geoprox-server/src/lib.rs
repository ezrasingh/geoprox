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
        .route("/geohash/", routing::get(handlers::encode_latlng))
        .route("/geohash/:ghash/", routing::get(handlers::decode_geohash))
        .route("/geohash/:ghash/neighbors/", routing::get(handlers::geohash_neighbors))
        .route("/shard/:index/", routing::post(handlers::create_index)
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

#[cfg(test)]
mod test {
    use crate::{dto, routes};
    use axum::Router;
    use axum_test::{TestServerConfig, TestServer};
    use serde_json::json;
    
    fn setup() -> TestServer {
        let app = Router::new().nest("/api/", routes());
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        TestServer::new_with_config(app, config).unwrap()
    }

    #[tokio::test]
    async fn test_geohash_api() {
        
        let server = setup();
        let req = server.get("/api/geohash/").add_query_params(dto::EncodeLatLng {
            lat: 45.0,
            lng: 45.0,
            depth: 6,
        });
        let res = req.await;
        
        println!("{:#?}", res);

        res.assert_status_ok();
        assert_eq!(res.header("cache-control").is_empty(), false);
        res.assert_json(&json!({
                "geohash": "v00000",
            })
        );

    }
}