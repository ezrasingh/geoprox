use crate::app::SharedState;
use crate::dto;
use axum::{extract, Json};
use geoprox_core::shard::GeoShardError;

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

pub async fn insert_resource(
    extract::State(app_state): extract::State<SharedState>,
    extract::Json(payload): extract::Json<dto::InsertResource>,
    extract::Path(index): extract::Path<String>
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
    let mut state = app_state.read().unwrap();

    match state.geoshard.query_range(&index, query.origin, &(query.range as f64)) {
        Ok(found) => Json(dto::QueryRangeResponse {
            found
        }),
        Err(err) => {
            panic!("query range search failed: {:#?}", err);
        },
    }
}