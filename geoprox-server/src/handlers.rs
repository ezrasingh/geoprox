pub mod geohash_api {
    use crate::app::{AppError, SharedState};
    use crate::dto::{
        DecodeGeohashResponse, EncodeLatLng, EncodeLatLngResponse, GeohashNeighborsResponse,
    };
    use anyhow::anyhow;
    use axum::{extract, Json};

    /// Decode geohash into coordinates.
    ///
    /// Decode geohash by path param, returns coordinates with precision estimates.
    #[utoipa::path(
        get,
        path = "/api/v1/geohash/{ghash}",
        params(
            ("ghash" = String, Path, description = "Geohash encoded region"),
        ),
        responses(
            (
                status = 200,
                description = "Object with latitude/longitude pair and precision errors",
                body = DecodeGeohashResponse
            )
        )
    )]
    pub async fn decode_geohash(
        extract::State(_state): extract::State<SharedState>,
        extract::Path(ghash): extract::Path<String>,
    ) -> Result<Json<DecodeGeohashResponse>, AppError> {
        match geoprox_core::geohash::decode(&ghash) {
            Ok((coord, lng_error, lat_error)) => Ok(Json(DecodeGeohashResponse {
                lat: coord.y,
                lng: coord.x,
                lat_error,
                lng_error,
            })),
            Err(err) => Err(anyhow!("could not decode geohash '{}': {:#?}", ghash, err).into()),
        }
    }

    /// Encode coordinates into geohash
    ///
    /// Encode coordinates by query params, returns geohash.
    #[utoipa::path(
        get,
        path = "/api/v1/geohash",
        params(EncodeLatLng),
        responses(
            (
                status = 200,
                description = "Object with geohash encoded latitude/longitude",
                body = EncodeLatLngResponse
            )
        )
    )]
    pub async fn encode_latlng(
        extract::State(_state): extract::State<SharedState>,
        extract::Query(payload): extract::Query<EncodeLatLng>,
    ) -> Result<Json<EncodeLatLngResponse>, AppError> {
        match geoprox_core::geohash::encode([payload.lng, payload.lat].into(), payload.depth) {
            Ok(geohash) => Ok(Json(EncodeLatLngResponse { geohash })),
            Err(err) => Err(anyhow!(
                "could not encode lat/lng ({}, {}): {:#?}",
                payload.lat,
                payload.lng,
                err
            )
            .into()),
        }
    }

    /// Neighboring regions
    ///
    /// Returns geohash neighbors in all cardinal directions.
    #[utoipa::path(
        get,
        path = "/api/v1/geohash/{ghash}/neighbors",
        params(
            ("ghash" = String, Path, description = "Geohash encoded region"),
        ),
        responses(
            (
                status = 200,
                description = "Object with geohash neighbors",
                body = GeohashNeighborsResponse
            )
        )
    )]
    pub async fn get_neighbors(
        extract::State(_state): extract::State<SharedState>,
        extract::Path(ghash): extract::Path<String>,
    ) -> Result<Json<GeohashNeighborsResponse>, AppError> {
        match geoprox_core::geohash::neighbors(&ghash) {
            Ok(neighbors) => Ok(Json(Into::<GeohashNeighborsResponse>::into(neighbors))),
            Err(err) => Err(anyhow!(
                "could not compute geohash neighbors for '{}': {:#?}",
                ghash,
                err
            )
            .into()),
        }
    }
}

pub mod geoshard_api {
    use crate::app::{AppError, SharedState};
    use crate::dto::{
        CreateIndexResponse, DropIndexResponse, InsertKey, InsertKeyResponse, KeysFound,
        QueryRange, QueryRangeResponse, RemoveKey, RemoveKeyResponse,
    };
    use anyhow::anyhow;
    use axum::{extract, Json};
    use geoprox_core::shard::GeoShardError;

    /// Create geospatial index
    ///
    /// Creates an in-memory index within this geoshard
    #[utoipa::path(
        post,
        path = "/api/v1/shard/{index}",
        responses(
            (
                status = 201,
                description = "Created an index",
                body = CreateIndexResponse
            )
        )
    )]
    pub async fn create_index(
        extract::State(state): extract::State<SharedState>,
        extract::Path(index): extract::Path<String>,
    ) -> Result<Json<CreateIndexResponse>, AppError> {
        let mut state = state.write().unwrap();

        match state.geoshard.create_index(&index) {
            Ok(_) => Ok(Json(CreateIndexResponse {
                created: true,
                existed: false,
            })),
            Err(err) => {
                if let GeoShardError::IndexAlreadyExists(_) = err {
                    Ok(Json(CreateIndexResponse {
                        created: false,
                        existed: true,
                    }))
                } else {
                    Err(anyhow!("could not create index '{}': {:#?}", index, err).into())
                }
            }
        }
    }

    /// Insert key into index
    ///
    /// Inserts key into geospatial index
    #[utoipa::path(
        put,
        path = "/api/v1/shard/{index}",
        responses(
            (
                status = 201,
                description = "Inserted key into index",
                body = InsertKeyResponse
            )
        )
    )]
    pub async fn insert_key(
        extract::State(state): extract::State<SharedState>,
        extract::Path(index): extract::Path<String>,
        extract::Json(payload): extract::Json<InsertKey>,
    ) -> Result<Json<InsertKeyResponse>, AppError> {
        let mut state = state.write().unwrap();

        match state
            .geoshard
            .insert_key(&index, &payload.key, &[payload.lat, payload.lng])
        {
            Ok(geohash) => Ok(Json(InsertKeyResponse {
                key: payload.key,
                geohash,
            })),
            Err(err) => Err(anyhow!(
                "could not insert key '{}' at index '{}': {:#?}",
                payload.key,
                index,
                err
            )
            .into()),
        }
    }

    /// Remove key from index
    ///
    /// Removed key from geospatial index
    #[utoipa::path(
        patch,
        path = "/api/v1/shard/{index}",
        responses(
            (
                status = 200,
                description = "Key removed from index",
                body = InsertKeyResponse
            )
        )
    )]
    pub async fn remove_key(
        extract::State(state): extract::State<SharedState>,
        extract::Path(index): extract::Path<String>,
        extract::Json(payload): extract::Json<RemoveKey>,
    ) -> Result<Json<RemoveKeyResponse>, AppError> {
        let mut state = state.write().unwrap();

        match state.geoshard.remove_key(&index, &payload.key) {
            Ok(deleted) => Ok(Json(RemoveKeyResponse {
                key: payload.key,
                deleted,
            })),
            Err(err) => Err(anyhow!(
                "could not remove key '{}' at index '{}': {:#?}",
                payload.key,
                index,
                err
            )
            .into()),
        }
    }

    /// Drop index
    ///
    /// Deletes geospatial index, all keys will be lost
    #[utoipa::path(
        delete,
        path = "/api/v1/shard/{index}",
        responses(
            (
                status = 202,
                description = "Index deleted",
                body = DropIndexResponse
            )
        )
    )]
    pub async fn drop_index(
        extract::State(state): extract::State<SharedState>,
        extract::Path(index): extract::Path<String>,
    ) -> Json<DropIndexResponse> {
        let mut state = state.write().unwrap();

        state.geoshard.drop_index(&index);

        Json(DropIndexResponse { deleted: true })
    }

    /// Search nearby
    ///
    /// Search geospatial index for all keys within some distance
    #[utoipa::path(
        get,
        path = "/api/v1/shard/{index}",
        params(QueryRange),
        responses(
            (
                status = 200,
                description = "Index deleted",
                body = QueryRangeResponse
            )
        )
    )]
    pub async fn query_range(
        extract::State(state): extract::State<SharedState>,
        extract::Path(index): extract::Path<String>,
        extract::Query(query): extract::Query<QueryRange>,
    ) -> Result<Json<QueryRangeResponse>, AppError> {
        let state = state.read().unwrap();

        match state
            .geoshard
            .query_range(&index, [query.lat, query.lng], &query.range.into())
        {
            Ok(found) => Ok(Json(QueryRangeResponse {
                found: KeysFound(found),
            })),
            Err(err) => Err(anyhow!("query range search failed: {:#?}", err).into()),
        }
    }
}
