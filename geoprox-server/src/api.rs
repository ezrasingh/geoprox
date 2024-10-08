use crate::app::{AppState, SharedState};
use crate::handlers::{geohash_api, geoshard_api};
use crate::middleware::{json_content, set_cache_control};
use axum::{
    routing::{get, post, put},
    Router,
};
use std::sync::Arc;
use std::time::Duration;

fn purge_expirations(state: SharedState, every: Duration) {
    tokio::spawn(async move {
        let mut timer = tokio::time::interval(every);
        loop {
            timer.tick().await;
            if let Ok(mut app) = state.try_write() {
                app.geoshard.purge_keys();
            }
        }
    });
}

fn persist_snapshot(state: SharedState, every: Duration) {
    tokio::spawn(async move {
        let mut timer = tokio::time::interval(every);
        loop {
            timer.tick().await;
            if let Ok(app) = state.try_read() {
                app.store_snapshot();
            }
        }
    });
}

/// Returns REST API router
pub fn routes(app_state: AppState) -> Router {
    let server_config = app_state.server_config.clone();
    let state = SharedState::from(app_state);

    // ? background tasks
    purge_expirations(Arc::clone(&state), Duration::from_secs(1));

    if !server_config.snapshots.disabled {
        persist_snapshot(
            Arc::clone(&state),
            server_config.snapshots.every.unwrap_or_default().into(),
        );
    }

    Router::new()
        .nest(
            "/geohash",
            Router::new()
                .route("/", get(geohash_api::encode_latlng))
                .route("/:ghash", get(geohash_api::decode_geohash))
                .route("/:ghash/neighbors", get(geohash_api::get_neighbors))
                .layer(axum::middleware::from_fn(set_cache_control)),
        )
        .route(
            "/shard/:index",
            post(geoshard_api::create_index)
                .delete(geoshard_api::drop_index)
                .put(geoshard_api::insert_key)
                .patch(geoshard_api::remove_key)
                .get(geoshard_api::query_range),
        )
        .route("/shard", get(geoshard_api::query_range_many))
        .route(
            "/shard/:index/batch",
            put(geoshard_api::insert_key_batch).patch(geoshard_api::remove_key_batch),
        )
        .layer(axum::middleware::from_fn(json_content))
        .with_state(Arc::clone(&state))
}

pub mod docs {
    use crate::dto;
    use crate::handlers::{geohash_api, geoshard_api};
    use axum::Router;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    #[derive(OpenApi)]
    #[openapi(
    paths(
        // Geohash API
        geohash_api::decode_geohash,
        geohash_api::encode_latlng,
        geohash_api::get_neighbors,
        // Geoshard API
        geoshard_api::create_index,
        geoshard_api::insert_key,
        geoshard_api::insert_key_batch,
        geoshard_api::remove_key,
        geoshard_api::remove_key_batch,
        geoshard_api::drop_index,
        geoshard_api::query_range,
        geoshard_api::query_range_many,
    ),
    components(schemas(
        dto::EncodeLatLng,
        dto::InsertKey,
        dto::InsertKeyBatch,
        dto::RemoveKey,
        dto::RemoveKeyBatch,
        dto::QueryRange,
        dto::QueryRangeMany,
        dto::DecodeGeohashResponse,
        dto::EncodeLatLngResponse,
        dto::GeohashNeighborsResponse,
        dto::CreateIndexResponse,
        dto::InsertKeyResponse,
        dto::InsertKeyBatchResponse,
        dto::RemoveKeyResponse,
        dto::RemoveKeyBatchResponse,
        dto::DropIndexResponse,
        dto::QueryRangeResponse,
        dto::QueryRangeManyResponse,
        geoprox_core::models::Neighbor,
    ), responses(
        dto::DecodeGeohashResponse,
        dto::EncodeLatLngResponse,
        dto::GeohashNeighborsResponse,
        dto::CreateIndexResponse,
        dto::InsertKeyResponse,
        dto::InsertKeyBatchResponse,
        dto::RemoveKeyResponse,
        dto::RemoveKeyBatchResponse,
        dto::DropIndexResponse,
        dto::QueryRangeResponse,
        dto::QueryRangeManyResponse,
    )),
    tags(
        (name = "Geoprox", description = "Geospatial index API")
    )
)]
    struct ApiDoc;

    /// Returns Swagger docs router
    pub fn router() -> Router {
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-spec/openapi.json", ApiDoc::openapi()))
    }

    pub fn openapi_spec_json(pretty_print: bool) -> Result<String, serde_json::Error> {
        if pretty_print {
            ApiDoc::openapi().to_pretty_json()
        } else {
            ApiDoc::openapi().to_json()
        }
    }
}

#[cfg(test)]
mod test {
    use std::env::current_dir;

    use super::*;
    use crate::{
        config::{ServerConfig, SnapshotConfig},
        dto,
    };
    use axum_test::{TestServer, TestServerConfig};
    use geoprox_core::models::GeoShardConfig;
    use serde_json::json;

    fn setup() -> TestServer {
        let app = AppState::new(
            ServerConfig {
                snapshots: SnapshotConfig {
                    disabled: false,
                    path: Some(current_dir().unwrap()),
                    every: None,
                },
                ..Default::default()
            },
            GeoShardConfig::default(),
        );
        let router = Router::new().nest("/api/v1/", routes(app));
        let config = TestServerConfig::builder().build();
        TestServer::new_with_config(router, config).unwrap()
    }

    #[tokio::test]
    async fn can_geohash_api() {
        let server = setup();
        let req = server
            .get("/api/v1/geohash")
            .add_query_params(dto::EncodeLatLng {
                lat: 45.0,
                lng: 45.0,
                depth: 6,
            });
        let res = req.await;

        res.assert_status_ok();
        assert_eq!(res.header("cache-control").is_empty(), false);
        assert_eq!(res.header("content-type"), "application/json");
        res.assert_json(&json!({
            "geohash": "v00000",
        }));
    }

    #[tokio::test]
    async fn can_range_query_api() {
        let server = setup();
        server.post("/api/v1/shard/drivers").await;
        server
            .put("/api/v1/shard/drivers")
            .json(&json!({ "key": "Alice", "lat": 1.0, "lng": 0.0 }))
            .await;
        server
            .put("/api/v1/shard/drivers")
            .json(&json!({ "key": "Bob", "lat": 0.0, "lng": 1.0 }))
            .await;
        let res = server
            .get("/api/v1/shard/drivers")
            .add_query_params(json!({
                "lat": 0.0, "lng": 0.0, "range": 1000
            }))
            .await;
        res.assert_status_ok();
        assert_eq!(res.header("content-type"), "application/json");
    }

    #[tokio::test]
    async fn can_invalidate_keys() {
        let server = setup();
        server.post("/api/v1/shard/drivers").await;
        server
            .put("/api/v1/shard/drivers")
            .json(&json!({ "key": "Alice", "lat": 0.0, "lng": 0.0, "ttl": 1 }))
            .await;
        server
            .put("/api/v1/shard/drivers")
            .json(&json!({ "key": "Bob", "lat": 0.0, "lng": 0.0, "ttl": 1  }))
            .await;
        let res = server
            .get("/api/v1/shard/drivers")
            .add_query_params(json!({
                "lat": 0.0, "lng": 0.0, "range": 1000
            }))
            .await;
        assert_eq!(res.json::<dto::QueryRangeResponse>().found.len(), 2);
        tokio::time::sleep(Duration::from_secs(2)).await;
        let res = server
            .get("/api/v1/shard/drivers")
            .add_query_params(json!({
                "lat": 0.0, "lng": 0.0, "range": 1000
            }))
            .await;
        assert_eq!(res.json::<dto::QueryRangeResponse>().found.len(), 0);
    }
}
