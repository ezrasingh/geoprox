use crate::app::{AppState, SharedState};
use crate::handlers::{geohash_api, geoshard_api};
use crate::middleware::set_cache_control;
use axum::{
    routing::{get, post},
    Router,
};
use geoprox_core::shard::GeoShardConfig;
use std::sync::Arc;

pub fn routes(shard_config: Option<GeoShardConfig>) -> Router {
    let state: SharedState = match shard_config {
        Some(config) => {
            let app = AppState::from(config);
            SharedState::from(app)
        }
        None => SharedState::default(),
    };

    Router::new()
        .nest(
            "/geohash/",
            Router::new()
                .route("/", get(geohash_api::encode_latlng))
                .route("/:ghash/", get(geohash_api::decode_geohash))
                .route("/:ghash/neighbors/", get(geohash_api::get_neighbors))
                .layer(axum::middleware::from_fn(set_cache_control)),
        )
        .route(
            "/shard/:index/",
            post(geoshard_api::create_index)
                .delete(geoshard_api::drop_index)
                .put(geoshard_api::insert_key)
                .get(geoshard_api::query_range),
        )
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
        geoshard_api::remove_key,
        geoshard_api::drop_index,
        geoshard_api::query_range,
    ),
    components(schemas(
        dto::EncodeLatLng,
        dto::InsertKey,
        dto::RemoveKey,
        dto::QueryRange,
        dto::KeysFound,
        dto::DecodeGeohashResponse,
        dto::EncodeLatLngResponse,
        dto::GeohashNeighborsResponse,
        dto::CreateIndexResponse,
        dto::InsertKeyResponse,
        dto::RemoveKeyResponse,
        dto::DropIndexResponse,
        dto::QueryRangeResponse,
    ), responses(
        dto::DecodeGeohashResponse,
        dto::EncodeLatLngResponse,
        dto::GeohashNeighborsResponse,
        dto::CreateIndexResponse,
        dto::InsertKeyResponse,
        dto::RemoveKeyResponse,
        dto::DropIndexResponse,
        dto::QueryRangeResponse,
    )),
    tags(
        (name = "GeoProx", description = "Geospatial index API")
    )
)]
    struct ApiDoc;

    pub fn router() -> Router {
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-spec/openapi.json", ApiDoc::openapi()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dto;
    use axum::Router;
    use axum_test::{TestServer, TestServerConfig};
    use serde_json::json;

    fn setup() -> TestServer {
        let app = Router::new().nest("/api/v1/", routes(None));
        let config = TestServerConfig::builder()
            .default_content_type("application/json")
            .build();
        TestServer::new_with_config(app, config).unwrap()
    }

    #[tokio::test]
    async fn test_geohash_api() {
        let server = setup();
        let req = server
            .get("/api/v1/geohash/")
            .add_query_params(dto::EncodeLatLng {
                lat: 45.0,
                lng: 45.0,
                depth: 6,
            });
        let res = req.await;

        res.assert_status_ok();
        assert_eq!(res.header("cache-control").is_empty(), false);
        res.assert_json(&json!({
            "geohash": "v00000",
        }));
    }

    #[tokio::test]
    async fn test_range_query_api() {
        let server = setup();
        server.post("/api/v1/shard/drivers/").await;
        server
            .put("/api/v1/shard/drivers/")
            .json(&json!({ "key": "Alice", "lat": 1.0, "lng": 0.0 }))
            .await;
        server
            .put("/api/v1/geoshard/drivers/")
            .json(&json!({ "key": "Bob", "lat": 0.0, "lng": 1.0 }))
            .await;
        let res = server
            .get("/api/v1/shard/drivers/")
            .add_query_params(json!({
                "lat": 0.0, "lng": 0.0, "range": 1000
            }))
            .await;
        res.assert_status_ok();
    }
}
