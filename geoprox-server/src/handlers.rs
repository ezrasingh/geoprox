use crate::app::SharedState;
use crate::dto;
use axum::{extract, Json};
use geoprox_core::shard::GeoShardError;

pub async fn decode_geohash(
    extract::State(_): extract::State<SharedState>,
    extract::Path(ghash): extract::Path<String>
) -> Json<dto::DecodeGeohashResponse> {
    match geoprox_core::geohash::decode(&ghash) {
        Ok((coord, lng_error, lat_error)) => Json(dto::DecodeGeohashResponse { 
            lat: coord.y, 
            lng: coord.x,
            lat_error, 
            lng_error
        }),
        Err(err) => {
            panic!("could not decode geohash '{}': {:#?}", ghash, err);
        },
    }
}

pub async fn encode_latlng(
    extract::State(_): extract::State<SharedState>,
    extract::Query(payload): extract::Query<dto::EncodeLatLng>,
) -> Json<dto::EncodeGeohashResponse> {
    match geoprox_core::geohash::encode([payload.lng, payload.lat].into(), payload.depth) {
        Ok(geohash) => Json(dto::EncodeGeohashResponse{
            geohash
        }),
        Err(err) => {
            panic!("could not encode lat/lng ({}, {}): {:#?}", payload.lat, payload.lng, err);
        },
    }
}

pub async fn geohash_neighbors(
    extract::State(_): extract::State<SharedState>,
    extract::Path(ghash): extract::Path<String>
) -> Json<dto::GeohashNeighborsResponse> {
    match geoprox_core::geohash::neighbors(&ghash) {
        Ok(neighbors) => Json(neighbors.into()),
        Err(err) => {
            panic!("could not compute geohash neighbors for '{}': {:#?}", ghash, err);
        },
    }
}

pub async fn create_index(
    extract::State(app_state): extract::State<SharedState>,
    extract::Path(index): extract::Path<String>
) -> Json<dto::CreateIndexResponse> {
    let mut state = app_state.write().unwrap();

    match state.geoshard.create_index(&index) {
        Ok(_) => Json(dto::CreateIndexResponse {
            created: true,
            existed: false
        }),
        Err(err) => {
            if let GeoShardError::IndexAlreadyExists(_) = err {
                Json(dto::CreateIndexResponse {
                    created: false,
                    existed: true
                })
            } else {
                panic!("could not create index '{}': {:#?}", index, err);
            }
        },
    }
}

pub async fn insert_key(
    extract::State(app_state): extract::State<SharedState>,
    extract::Path(index): extract::Path<String>,
    extract::Json(payload): extract::Json<dto::InsertResource>,
) -> Json<dto::InsertResourceResponse> {
    let mut state = app_state.write().unwrap();

    match state.geoshard.insert_key(&index, &payload.key, &payload.position) {
        Ok(geohash) => Json(dto::InsertResourceResponse {
            key: payload.key,
            geohash
        }),
        Err(err) => {
            panic!("could not insert key '{}' at index '{}': {:#?}", payload.key, index, err);
        },
    }
}

pub async fn drop_index(
    extract::State(app_state): extract::State<SharedState>,
    extract::Path(index): extract::Path<String>
) -> Json<dto::DropIndexResponse> {
    let mut state = app_state.write().unwrap();

    state.geoshard.drop_index(&index);

    Json(dto::DropIndexResponse {
        deleted: true
    })
}

pub async fn query_range(
    extract::State(app_state): extract::State<SharedState>,
    extract::Path(index): extract::Path<String>,
    extract::Query(query): extract::Query<dto::QueryRange>
) -> Json<dto::QueryRangeResponse> {
    let state = app_state.read().unwrap();

    match state.geoshard.query_range(&index, [query.lat, query.lng], &query.range.into()) {
        Ok(found) => Json(dto::QueryRangeResponse {
            found
        }),
        Err(err) => {
            panic!("query range search failed: {:#?}", err);
        },
    }
}