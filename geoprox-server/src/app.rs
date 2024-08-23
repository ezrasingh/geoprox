use axum::{
    response::{IntoResponse, Response},
    Json,
};
use std::fs;
use std::sync::{Arc, RwLock};

use geoprox_core::models::GeoShardConfig;
use geoprox_core::shard::GeoShard;

use crate::config::ServerConfig;
use crate::dto::AppErrorResponse;

#[derive(Clone, Default)]
pub struct AppState {
    pub geoshard: GeoShard,
    pub server_config: ServerConfig,
    pub shard_config: GeoShardConfig,
}

impl AppState {
    pub fn new(server_config: ServerConfig, shard_config: GeoShardConfig) -> Self {
        match fs::read(server_config.snapshots.bin_path()) {
            Ok(state) => Self {
                geoshard: bincode::deserialize(&state).expect("could not restore snapshot"),
                server_config,
                shard_config,
            },
            Err(_) => Self {
                geoshard: GeoShard::from(shard_config.clone()),
                server_config,
                shard_config,
            },
        }
    }

    pub fn store_snapshot(&self) {
        fs::write(
            self.server_config.snapshots.bin_path(),
            bincode::serialize(&self.geoshard).unwrap(),
        )
        .expect("could not store snapshot");
    }
}

pub type SharedState = Arc<RwLock<AppState>>;

impl From<AppState> for SharedState {
    fn from(app: AppState) -> Self {
        Arc::new(app.into())
    }
}

pub struct AppError(pub anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppErrorResponse {
                message: self.0.to_string(),
            }),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
